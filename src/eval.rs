/*!
# Evaluator

Evaluates a parsed Monkey program using a tree-walking evaluation strategy,
interpreting the parsed AST representation of the source code "on the fly."
*/
pub(crate) mod environment;
pub mod error;
pub(crate) mod object;

use std::{cell::RefCell, rc::Rc};

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
        None => Err(error::EvaluationError::new(format!(
            "identifier not found: {}",
            ident
        ))),
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
        _ => Err(error::EvaluationError::new(format!(
            "type mismatch: {} {} {}",
            left, operator, right
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
        token::Token::Slash => Ok(Rc::new(object::Object::Integer(left_int / right_int))),
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
            ("5 + true;", "type mismatch: 5 + true"),
            ("5 + true; 5;", "type mismatch: 5 + true"),
            ("-true", "unknown operator: -true"),
            ("true + false;", "unknown operator: true + false"),
            ("5; true + false; 5", "unknown operator: true + false"),
            (
                "if (10 > 1) { true + false; )",
                "unknown operator: true + false",
            ),
            ("foobar", "identifier not found: foobar"),
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
}
