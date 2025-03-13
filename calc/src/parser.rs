use lalrpop_util::lalrpop_mod;

use crate::syntax::Expr;

lalrpop_mod!(pub grammar);

pub fn parse_expr(input: &str) -> Result<Expr, String> {
    match grammar::ExprParser::new().parse(input) {
        Ok(expr) => Ok(*expr),
        Err(e) => Err(e.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_const() {
        assert_eq!(Expr::Const(42), parse_expr("42").unwrap());
    }

    #[test]
    fn test_parse_var() {
        assert_eq!(Expr::Var(String::from("x")), parse_expr("x").unwrap());
    }

    #[test]
    fn test_parse_ops() {
        let actual = parse_expr("1 + 2 * 3").unwrap();
        let expected = Expr::Add(
            Box::new(Expr::Const(1)),
            Box::new(Expr::Mul(Box::new(Expr::Const(2)), Box::new(Expr::Const(3)))),
        );
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_precedence() {
        let actual = parse_expr("1 * 2 + 3").unwrap();
        let expected = Expr::Add(
            Box::new(Expr::Mul(Box::new(Expr::Const(1)), Box::new(Expr::Const(2)))),
            Box::new(Expr::Const(3)),
        );
        assert_eq!(expected, actual);
    }
}
