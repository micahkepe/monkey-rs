/*!
# Evaluator

Evaluates a parsed Monkey program using a tree-walking evaluation strategy,
interpreting the parsed AST representation of the source code "on the fly."
*/
/* Modules */
pub(crate) mod builtin;
pub mod environment;
pub mod error;
pub(crate) mod object;

/* Re-exports */
pub use builtin::Builtin;

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{parser::ast, token};

/// Evaluate a parsed Monkey AST node and return its corresponding object
/// representation.
pub fn eval(
    node: ast::Node,
    env: &environment::Env,
) -> Result<Rc<object::Object>, error::EvaluationError> {
    match node {
        /* Statements */
        ast::Node::Program(program) => eval_program(&program, env),
        ast::Node::Stmt(statement) => eval_statement(&statement, env),
        /* Expressions */
        ast::Node::Expr(expression) => eval_expression(&expression, env),
    }
}

/// Returns whether the given object is "truthy."
fn is_truthy(object: &object::Object) -> bool {
    !matches!(
        *object,
        object::Object::Boolean(false) | object::Object::Null
    )
}

/// Evaluate a parsed Monkey AST expression node and return its corresponding
/// object representation.
fn eval_expression(
    expression: &ast::Expression,
    env: &environment::Env,
) -> Result<Rc<object::Object>, error::EvaluationError> {
    match expression {
        ast::Expression::Identifier(ident) => eval_identifier(ident, env),
        ast::Expression::Lit(ast::Literal::Integer(value)) => {
            Ok(Rc::new(object::Object::Integer(*value as i64)))
        }
        ast::Expression::Lit(ast::Literal::Boolean(value)) => {
            Ok(Rc::new(object::Object::Boolean(*value)))
        }
        ast::Expression::Lit(ast::Literal::String(value)) => {
            Ok(Rc::new(object::Object::String(value.clone())))
        }
        ast::Expression::Lit(ast::Literal::Array(arr)) => {
            let list = eval_expressions(arr, &Rc::clone(env))?;
            Ok(Rc::new(object::Object::Array(list)))
        }
        ast::Expression::Lit(ast::Literal::Hash(entries)) => {
            let hash = eval_hash_literal(entries, &Rc::clone(env))?;
            Ok(Rc::new(object::Object::Hash(hash)))
        }
        ast::Expression::Prefix(operator, expression) => {
            let right = eval_expression(expression, env)?;
            eval_prefix_expression(operator, &right)
        }
        ast::Expression::Infix(operator, left, right) => {
            let left = eval_expression(left, &Rc::clone(env))?;
            let right = eval_expression(right, env)?;
            eval_infix_expression(operator, &left, &right)
        }
        ast::Expression::If(condition, consequence, alternative) => {
            let condition = eval_expression(condition, &Rc::clone(env))?;

            if is_truthy(&condition) {
                eval_block_statement(consequence, env)
            } else {
                match alternative {
                    Some(alt) => eval_block_statement(alt, env),
                    None => Ok(Rc::new(object::Object::Null)),
                }
            }
        }
        ast::Expression::Fn(params, body) => Ok(Rc::new(object::Object::Function(
            params.clone(),
            body.clone(),
            Rc::clone(env),
        ))),
        ast::Expression::Call(func, args) => {
            let func = eval_expression(func, &Rc::clone(env))?;
            let args = eval_expressions(args, env)?;
            apply_function(&func, &args)
        }
        ast::Expression::Index(left, index) => {
            // Evaluate both expressions first before evaluating indexing.
            let left_expr = eval_expression(left, &Rc::clone(env))?;
            let index_expr = eval_expression(index, &Rc::clone(env))?;
            eval_index_expression(&left_expr, &index_expr)
        }
    }
}

/// Evaluate the hash literal expression with the given (key, value) expression
/// entries.
fn eval_hash_literal(
    entries: &[(ast::Expression, ast::Expression)],
    env: &environment::Env,
) -> Result<HashMap<Rc<object::HashableObject>, Rc<object::Object>>, error::EvaluationError> {
    let mut hash = HashMap::new();

    for (key_expr, value_expr) in entries {
        let key_obj = eval_expression(key_expr, env)?;

        // Verify that key object is hashable
        let hash_key = match key_obj.as_hashable() {
            Some(k) => Rc::new(k),
            None => {
                return Err(error::EvaluationError::new(format!(
                    "unusable as hash key: {}",
                    key_obj
                )))
            }
        };

        let value_obj = eval_expression(value_expr, env)?;
        hash.insert(hash_key, value_obj);
    }

    Ok(hash)
}

/// Evaluate the index expression with the given left and index expressions.
fn eval_index_expression(
    left_expr: &Rc<object::Object>,
    index_expr: &Rc<object::Object>,
) -> Result<Rc<object::Object>, error::EvaluationError> {
    match (&**left_expr, &**index_expr) {
        (object::Object::Array(arr), object::Object::Integer(idx)) => {
            eval_array_index_expression(arr, *idx)
        }
        (object::Object::Hash(hash), key) => eval_hash_index_expression(hash, key),
        _ => Err(error::EvaluationError::new(format!(
            "index operator not supported: {}",
            index_expr
        ))),
    }
}

/// Evaluate the hash index expression with the given hash object and index
/// expression.
fn eval_hash_index_expression(
    hash: &HashMap<Rc<object::HashableObject>, Rc<object::Object>>,
    key: &object::Object,
) -> Result<Rc<object::Object>, error::EvaluationError> {
    let hash_key = match key.as_hashable() {
        Some(k) => &Rc::new(k),
        None => {
            return Err(error::EvaluationError::new(format!(
                "unusable as hash key: {}",
                key
            )))
        }
    };

    match hash.get(hash_key) {
        Some(val) => Ok(Rc::clone(val)),
        None => Ok(Rc::new(object::Object::Null)),
    }
}

/// Evaluate the array index expression from the given array object and index
fn eval_array_index_expression(
    arr: &[Rc<object::Object>],
    idx: i64,
) -> Result<Rc<object::Object>, error::EvaluationError> {
    let max = (arr.len() as i64) - 1;

    if idx < 0 || idx > max {
        Ok(Rc::new(object::Object::Null))
    } else {
        let obj = arr.get(idx as usize).expect("Index out of bounds");
        Ok(Rc::clone(obj))
    }
}

/// Apply the function with the given arguments, returning an error with the
/// function cannot be applied. The function and its arguments are evaluated
/// within a new enclosed environment to run in isolation.
fn apply_function(
    func: &Rc<object::Object>,
    args: &[Rc<object::Object>],
) -> Result<Rc<object::Object>, error::EvaluationError> {
    match &**func {
        object::Object::Function(params, body, env) => {
            let mut env = environment::Environment::new_enclosed_environment(&Rc::clone(env));

            // Check that the number of parameters passed matches the expected
            // number of arguments
            if params.len() != args.len() {
                return Err(error::EvaluationError::new(format!(
                    "invalid number of arguments: expected={}, got={}",
                    params.len(),
                    args.len()
                )));
            }

            // Store the parameter values
            for (i, param) in params.iter().enumerate() {
                env.set(param, args[i].clone());
            }

            let evaluated = eval_block_statement(body, &Rc::new(RefCell::new(env)))?;
            unwrap_return_value(evaluated)
        }
        object::Object::Builtin(func) => func.apply(args),
        other => Err(error::EvaluationError::new(format!(
            "not a function: {}",
            other
        ))),
    }
}

/// Unwraps the result of an environment, which prevents the bubbling up of the
/// return. This is necessary so that only the evaluation of the last-called
/// function's body is stopped.
fn unwrap_return_value(
    object: Rc<object::Object>,
) -> Result<Rc<object::Object>, error::EvaluationError> {
    if let object::Object::ReturnValue(val) = &*object {
        Ok(Rc::clone(val))
    } else {
        Ok(object)
    }
}

/// Evaluate a series of expressions, returning the results of the expressions
/// by index in an array. Expressions are evaluated from left-to-right.
fn eval_expressions(
    expressions: &[ast::Expression],
    env: &environment::Env,
) -> Result<Vec<Rc<object::Object>>, error::EvaluationError> {
    let mut result = Vec::new();

    for expr in expressions {
        let val = eval_expression(expr, env)?;
        result.push(val);
    }

    Ok(result)
}

/// Evaluate identifier expression.
fn eval_identifier(
    ident: &str,
    env: &environment::Env,
) -> Result<Rc<object::Object>, error::EvaluationError> {
    match env.borrow().get(ident) {
        Some(obj) => Ok(obj.clone()),
        None => match Builtin::lookup(ident) {
            Some(obj) => Ok(Rc::new(obj)),
            None => Err(error::EvaluationError::new(format!(
                "identifier not found: {}",
                ident
            ))),
        },
    }
}

/// Evaluate statements within a block statement.
fn eval_block_statement(
    statements: &[ast::Statement],
    env: &environment::Env,
) -> Result<Rc<object::Object>, error::EvaluationError> {
    let mut result = Rc::new(object::Object::Null);

    for stmt in statements {
        result = eval_statement(stmt, env)?;

        match *result {
            object::Object::ReturnValue(_) => return Ok(result),
            _ => continue,
        }
    }

    Ok(result)
}

/// Evaluates the given infix expression from its operator, and left and right
/// expressions.
fn eval_infix_expression(
    operator: &token::Token,
    left: &Rc<object::Object>,
    right: &Rc<object::Object>,
) -> Result<Rc<object::Object>, error::EvaluationError> {
    match (&**left, &**right) {
        (object::Object::Integer(left_int), object::Object::Integer(right_int)) => {
            eval_integer_infix_expression(operator, *left_int, *right_int)
        }
        (object::Object::Boolean(left_b), object::Object::Boolean(right_b)) => {
            eval_boolean_infix_expression(operator, *left_b, *right_b)
        }
        (object::Object::String(left_str), object::Object::String(right_str)) => {
            eval_string_infix_expression(operator, left_str, right_str)
        }
        _ => Err(error::EvaluationError::new(format!(
            "unknown operator: {} {} {}",
            left, operator, right
        ))),
    }
}

/// Evaluates the given string infix expression from the left and right
/// expressions and the infix operator. Supported string operations are
/// comparison and concatenation.
fn eval_string_infix_expression(
    operator: &token::Token,
    left_str: &str,
    right_str: &str,
) -> Result<Rc<object::Object>, error::EvaluationError> {
    match operator {
        token::Token::Plus => Ok(Rc::new(object::Object::String(
            left_str.to_string() + right_str,
        ))),
        token::Token::Eq => Ok(Rc::new(object::Object::Boolean(left_str == right_str))),
        token::Token::NotEq => Ok(Rc::new(object::Object::Boolean(left_str != right_str))),
        _ => Err(error::EvaluationError::new(format!(
            "unknown operator: {} {} {}",
            left_str, operator, right_str
        ))),
    }
}

/// Evaluates the given Boolean infix expression from the left and right
/// expressions and the Boolean logical operator.
fn eval_boolean_infix_expression(
    operator: &token::Token,
    left_b: bool,
    right_b: bool,
) -> Result<Rc<object::Object>, error::EvaluationError> {
    match operator {
        token::Token::Eq => Ok(Rc::new(object::Object::Boolean(left_b == right_b))),
        token::Token::NotEq => Ok(Rc::new(object::Object::Boolean(left_b != right_b))),
        _ => Err(error::EvaluationError::new(format!(
            "unknown operator: {} {} {}",
            left_b, operator, right_b
        ))),
    }
}

/// Evaluates the given integer infix expression from the left and right
/// expressions and the infix arithmetic or logical operator.
fn eval_integer_infix_expression(
    operator: &token::Token,
    left_int: i64,
    right_int: i64,
) -> Result<Rc<object::Object>, error::EvaluationError> {
    match operator {
        /* Arithmetic operators */
        token::Token::Plus => Ok(Rc::new(object::Object::Integer(left_int + right_int))),
        token::Token::Minus => Ok(Rc::new(object::Object::Integer(left_int - right_int))),
        token::Token::Asterisk => Ok(Rc::new(object::Object::Integer(left_int * right_int))),
        token::Token::Slash => match right_int {
            0 => Err(error::EvaluationError::new("division by zero".to_string())),
            _ => Ok(Rc::new(object::Object::Integer(left_int / right_int))),
        },
        /* Logical operators */
        token::Token::Gt => Ok(Rc::new(object::Object::Boolean(left_int > right_int))),
        token::Token::Lt => Ok(Rc::new(object::Object::Boolean(left_int < right_int))),
        token::Token::Eq => Ok(Rc::new(object::Object::Boolean(left_int == right_int))),
        token::Token::NotEq => Ok(Rc::new(object::Object::Boolean(left_int != right_int))),
        _ => Err(error::EvaluationError::new(format!(
            "unknown operator: {} {} {}",
            left_int, operator, right_int
        ))),
    }
}

/// Evaluates the given prefix expression from its operator and right
/// expression.
fn eval_prefix_expression(
    operator: &token::Token,
    right: &Rc<object::Object>,
) -> Result<Rc<object::Object>, error::EvaluationError> {
    match operator {
        token::Token::Bang => eval_bang_operator_expression(right),
        token::Token::Minus => eval_minus_operator_expression(right),
        _ => Err(error::EvaluationError::new(format!(
            "unknown operator: {}{}",
            operator, right
        ))),
    }
}

/// Evaluates a minus operator expression from the right expression that the
/// bang is being applied to.
fn eval_minus_operator_expression(
    right: &Rc<object::Object>,
) -> Result<Rc<object::Object>, error::EvaluationError> {
    match **right {
        object::Object::Integer(int) => Ok(Rc::new(object::Object::Integer(-int))),
        _ => Err(error::EvaluationError::new(format!(
            "unknown operator: -{}",
            right
        ))),
    }
}

/// Evaluates a bang operator expression from the right expression that the
/// bang is being applied to.
fn eval_bang_operator_expression(
    right: &Rc<object::Object>,
) -> Result<Rc<object::Object>, error::EvaluationError> {
    match **right {
        object::Object::Boolean(b) => Ok(Rc::new(object::Object::Boolean(!b))),
        object::Object::Null => Ok(Rc::new(object::Object::Boolean(true))),
        _ => Ok(Rc::new(object::Object::Boolean(false))),
    }
}

/// Evaluate a parsed Monkey AST statement node and return its corresponding
/// object representation.
fn eval_statement(
    statement: &ast::Statement,
    env: &environment::Env,
) -> Result<Rc<object::Object>, error::EvaluationError> {
    match statement {
        ast::Statement::Expr(expr) => eval_expression(expr, &Rc::clone(env)),
        ast::Statement::Let(ident, expr) => {
            let val = eval_expression(expr, &Rc::clone(env))?;
            let obj = Rc::clone(&val);

            // Store value in environment
            env.borrow_mut().set(ident, obj);

            Ok(val)
        }
        ast::Statement::Return(expr) => {
            let val = eval_expression(expr, env)?;
            Ok(Rc::new(object::Object::ReturnValue(val)))
        }
    }
}

/// Evaluate parsed Monkey AST statements and return their corresponding
/// object representation.
fn eval_program(
    program: &[ast::Statement],
    env: &environment::Env,
) -> Result<Rc<object::Object>, error::EvaluationError> {
    let mut result = Rc::new(object::Object::Null);

    for stmt in program {
        result = eval_statement(stmt, &Rc::clone(env))?;

        // Return early if encounter a return statement
        match *result {
            object::Object::ReturnValue(_) => return Ok(result),
            _ => continue,
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::*;
    use crate::parser::*;

    /// Checks if the result of evaluating the input matches its expected value
    /// for each case in the provided case (input, expected) tuples.
    fn check_eval_case(cases: &[(&str, &str)]) {
        let env: environment::Env = Rc::new(RefCell::new(Default::default()));

        for (input, expected) in cases {
            match parse(input) {
                Ok(node) => match eval(node, &Rc::clone(&env)) {
                    Ok(eval) => assert_eq!(expected, &format!("{}", eval)),
                    Err(e) => assert_eq!(expected, &format!("{}", e)),
                },
                Err(e) => panic!("Parse error: {}", e),
            }
        }
    }

    #[test]
    fn test_eval_integer_expression() {
        let int_cases = [
            ("5", "5"),
            ("10", "10"),
            ("-5", "-5"),
            ("-10", "-10"),
            ("5 + 5 + 5 + 5 - 10", "10"),
            ("2 * 2 * 2 * 2 * 2", "32"),
            ("-50 + 100 + -50", "0"),
            ("5 * 2 + 10", "20"),
            ("5 + 2 * 10", "25"),
            ("20 + 2 * -10", "0"),
            ("50 / 2 * 2 + 10", "60"),
            ("2 * (5 + 10)", "30"),
            ("3 * 3 * 3 + 10", "37"),
            ("3 * (3 * 3) + 10", "37"),
            ("(5 + 10 * 2 + 15 / 3) * 2 + -10", "50"),
        ];
        check_eval_case(&int_cases);
    }

    #[test]
    fn test_eval_boolean_expression() {
        let int_cases = [
            ("true", "true"),
            ("false", "false"),
            ("1 < 2", "true"),
            ("1 > 2", "false"),
            ("1 < 1", "false"),
            ("1 > 1", "false"),
            ("1 == 1", "true"),
            ("1 != 1", "false"),
            ("1 == 2", "false"),
            ("1 != 2", "true"),
            ("true == true", "true"),
            ("false == false", "true"),
            ("true == false", "false"),
            ("true != false", "true"),
            ("false != true", "true"),
            ("(1 < 2) == true", "true"),
            ("(1 < 2) == false", "false"),
            ("(1 > 2) == true", "false"),
            ("(1 > 2) == false", "true"),
        ];
        check_eval_case(&int_cases);
    }

    #[test]
    fn test_bang_operator() {
        let bang_cases = [
            ("!true", "false"),
            ("!false", "true"),
            ("!5", "false"),
            ("!!true", "true"),
            ("!!false", "false"),
            ("!!5", "true"),
        ];
        check_eval_case(&bang_cases);
    }

    #[test]
    fn test_if_else_expressions() {
        let if_else_cases = [
            ("if (true) { 10 }", "10"),
            ("if (false) { 10 }", "null"),
            ("if (1) { 10 }", "10"),
            ("if (1 < 2) { 10 }", "10"),
            ("if (1 > 2) { 10 }", "null"),
            ("if (1 > 2) { 10 } else { 20 }", "20"),
            ("if (1 < 2) { 10 } else { 20 }", "10"),
        ];
        check_eval_case(&if_else_cases);
    }

    #[test]
    fn test_return_statements() {
        let return_cases = [
            ("return 10;", "10"),
            ("return 10; 9;", "10"),
            ("return 2 * 5; 9;", "10"),
            ("9; return 2 * 5; 9;", "10"),
            (
                "if (10 > 1) {\
                    if (10 > 1) {\
                        return 10;\
                    }\
                    \
                    return 1;\
             }",
                "10",
            ),
        ];
        check_eval_case(&return_cases);
    }

    #[test]
    fn test_error_handling() {
        let error_cases = [
            ("5 + true;", "unknown operator: 5 + true"),
            ("5 + true; 5;", "unknown operator: 5 + true"),
            ("-true", "unknown operator: -true"),
            ("true + false;", "unknown operator: true + false"),
            ("5; true + false; 5", "unknown operator: true + false"),
            (
                "if (10 > 1) { true + false; )",
                "unknown operator: true + false",
            ),
            ("foobar", "identifier not found: foobar"),
            ("\"Hello\" - \"World\"", "unknown operator: Hello - World"),
        ];
        check_eval_case(&error_cases);
    }

    #[test]
    fn test_let_statement() {
        let let_stmts = [
            ("let a = 5; a;", "5"),
            ("let a = 5 * 5; a;", "25"),
            ("let a = 5; let b = a; b;", "5"),
            ("let a = 5; let b = a; let c = a + b + 5; c;", "15"),
        ];
        check_eval_case(&let_stmts);
    }

    #[test]
    fn test_function_object() {
        let func_objs = [("fn(x) { x + 2; }", "fn(x) {\n (x + 2) \n}")];
        check_eval_case(&func_objs);
    }

    #[test]
    fn test_function_application() {
        let func_apps = [
            ("let identity = fn(x) { x; }; identity(5);", "5"),
            ("let identity = fn(x) { return x; }; identity(5);", "5"),
            ("let double = fn(x) { x * 2; }; double(5);", "10"),
            ("let add = fn(x, y) { x + y; }; add(5, 5);", "10"),
            (
                "let add = fn(x, y) { x + y; }; add(5 + 5, add(5, 5));",
                "20",
            ),
            ("fn(x) { x; }(5)", "5"),
        ];
        check_eval_case(&func_apps);
    }

    #[test]
    fn test_closures() {
        let input = [(
            "
            let newAdder = fn(x) {\
                fn(y) { x + y };\
            };\
            \
            let addTwo = newAdder(2);\
            addTwo(2);",
            "4",
        )];
        check_eval_case(&input);
    }

    #[test]
    fn test_string_literal() {
        let input = [("\"Hello World!\"", "Hello World!")];
        check_eval_case(&input);
    }

    #[test]
    fn test_string_concatenation() {
        let input = [("\"Hello\" + \" \" + \"World!\"", "Hello World!")];
        check_eval_case(&input);
    }

    #[test]
    fn test_builtin_functions() {
        let cases = [
            (r#"len("")"#, "0"),
            (r#"len("four")"#, "4"),
            (r#"len("hello world")"#, "11"),
            ("len(1)", "argument to `len` not supported, got 1"),
            (
                r#"len("one", "two")"#,
                "wrong number of arguments: expected=1, got=2",
            ),
            (r#"len([])"#, "0"),
        ];
        check_eval_case(&cases);
    }

    #[test]
    fn test_array_literals() {
        let cases = [("[1, 2 * 2, 3 + 3]", "[1, 4, 6]")];
        check_eval_case(&cases);
    }

    #[test]
    fn test_array_index_expressions() {
        let index_cases = [
            ("[1, 2, 3][0]", "1"),
            ("[1, 2, 3][1]", "2"),
            ("[1, 2, 3][2]", "3"),
            ("let i = 0; [1][i];", "1"),
            ("[1, 2, 3][1 + 1];", "3"),
            ("let myArray = [1, 2, 3]; myArray[2];", "3"),
            (
                "let myArray = [1, 2, 3]; myArray[0] + myArray[1] + myArray[2];",
                "6",
            ),
            (
                "let myArray = [1, 2, 3]; let i = myArray[0]; myArray[i]",
                "2",
            ),
            ("[1, 2, 3][3]", "null"),
            ("[1, 2, 3][-1]", "null"),
            ("first([1, 2, 3])", "1"),
            ("first([])", "null"),
            ("first(1)", "argument to `first` must be ARRAY, got 1"),
            ("last([1, 2, 3])", "3"),
            ("last([])", "null"),
            ("last(1)", "argument to `last` must be ARRAY, got 1"),
            ("rest([1, 2, 3])", "[2, 3]"),
            ("rest([])", "null"),
            ("push([], 1)", "[1]"),
            ("push(1, 1)", "argument to `push` must be ARRAY, got 1"),
        ];
        check_eval_case(&index_cases);
    }

    #[test]
    fn test_hash_literals() {
        let input = r#"
        let two = "two";
        {
            "one": 10 - 9,
            two: 1 + 1,
            "thr" + "ee": 6 / 2,
            4: 4,
            true: 5,
            false: 6
        }
    "#;

        let env: environment::Env = Rc::new(RefCell::new(Default::default()));
        let node = parse(input).expect("failed to parse input");
        let result = eval(node, &env).expect("evaluation failed");

        let expected: Vec<(object::HashableObject, object::Object)> = vec![
            (
                object::HashableObject::String("one".to_string()),
                object::Object::Integer(1),
            ),
            (
                object::HashableObject::String("two".to_string()),
                object::Object::Integer(2),
            ),
            (
                object::HashableObject::String("three".to_string()),
                object::Object::Integer(3),
            ),
            (
                object::HashableObject::Integer(4),
                object::Object::Integer(4),
            ),
            (
                object::HashableObject::Boolean(true),
                object::Object::Integer(5),
            ),
            (
                object::HashableObject::Boolean(false),
                object::Object::Integer(6),
            ),
        ];

        match &*result {
            object::Object::Hash(actual_map) => {
                assert_eq!(actual_map.len(), expected.len());

                for (expected_key, expected_val) in expected {
                    let key_rc = Rc::new(expected_key);
                    let actual_val = actual_map.get(&key_rc);
                    assert!(
                        actual_val.is_some(),
                        "expected key {:?} not found in hash",
                        key_rc
                    );

                    let actual_val = actual_val.unwrap();
                    assert_eq!(
                        &**actual_val, &expected_val,
                        "value mismatch for key {:?}",
                        key_rc
                    );
                }
            }
            other => panic!("expected Object::Hash, got {:?}", other),
        }
    }

    #[test]
    fn test_hash_index_expressions() {
        let cases = [
            (r#"{"foo": 5}["foo"]"#, "5"),
            (r#"{"foo": 5}["bar"]"#, "null"),
            (r#"let key = "foo"; {"foo": 5}[key]"#, "5"),
            (r#"{}["foo"]"#, "null"),
            (r#"{5: 5}[5]"#, "5"),
            (r#"{true: 5}[true]"#, "5"),
            (r#"{false: 5}[false]"#, "5"),
        ];
        check_eval_case(&cases);
    }
}
