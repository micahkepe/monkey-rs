/*!
# Evaluator

Evaluates a parsed Monkey program using a tree-walking evaluation strategy,
interpreting the parsed AST representation of the source code "on the fly."
*/
pub mod error;
pub(crate) mod object;

use std::rc::Rc;

use crate::{parser::ast, token};

/// Evaluate a parsed Monkey AST node and return its corresponding object
/// representation.
pub fn eval(node: ast::Node) -> Result<Rc<object::Object>, error::EvaluationError> {
    match node {
        /* Statements */
        ast::Node::Program(statements) => eval_statements(&statements),
        ast::Node::Stmt(statement) => eval_statement(&statement),
        /* Expressions */
        ast::Node::Expr(expression) => eval_expression(&expression),
    }
}

/// Evaluate a parsed Monkey AST expression node and return its corresponding
/// object representation.
fn eval_expression(
    expression: &ast::Expression,
) -> Result<Rc<object::Object>, error::EvaluationError> {
    match expression {
        ast::Expression::Lit(ast::Literal::Integer(value)) => {
            Ok(Rc::new(object::Object::Integer(*value as i64)))
        }
        ast::Expression::Lit(ast::Literal::Boolean(value)) => {
            Ok(Rc::new(object::Object::Boolean(*value)))
        }
        ast::Expression::Prefix(operator, expression) => {
            let right = eval_expression(expression)?;
            eval_prefix_expression(operator, &right)
        }
        _ => Ok(Rc::new(object::Object::Null)),
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
) -> Result<Rc<object::Object>, error::EvaluationError> {
    match statement {
        ast::Statement::Expr(expr) => eval_expression(expr),
        _ => Ok(Rc::new(object::Object::Null)),
    }
}

/// Evaluate parsed Monkey AST statements and return their corresponding
/// object representation.
fn eval_statements(
    program: &[ast::Statement],
) -> Result<Rc<object::Object>, error::EvaluationError> {
    let mut result = Rc::new(object::Object::Null);

    for stmt in program {
        result = eval_statement(stmt)?;
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::*;

    /// Checks if the result of evaluating the input matches its expected value
    /// for each case in the provided case (input, expected) tuples.
    fn check_eval_case(cases: &[(&str, &str)]) {
        for (input, expected) in cases {
            match parse(input) {
                Ok(node) => match eval(node) {
                    Ok(eval) => assert_eq!(expected, &format!("{}", eval)),
                    Err(e) => assert_eq!(expected, &format!("{}", e)),
                },
                Err(e) => panic!("Parse error: {}", e),
            }
        }
    }

    #[test]
    fn test_eval_integer_expression() {
        let int_cases = [("5", "5"), ("10", "10"), ("-5", "-5"), ("-10", "-10")];
        check_eval_case(&int_cases);
    }

    #[test]
    fn test_eval_boolean_expression() {
        let int_cases = [("true", "true"), ("false", "false")];
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
}
