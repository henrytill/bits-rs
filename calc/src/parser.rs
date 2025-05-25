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
        let expected = Expr::add(Expr::Const(1), Expr::mul(Expr::Const(2), Expr::Const(3)));
        let actual = parse_expr("1 + 2 * 3").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_precedence() {
        let expected = Expr::add(Expr::mul(Expr::Const(1), Expr::Const(2)), Expr::Const(3));
        let actual = parse_expr("1 * 2 + 3").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_sub_double_negation() {
        let expected = Expr::sub(
            Expr::Var(String::from("x")),
            Expr::neg(Expr::neg(Expr::Var(String::from("x")))),
        );
        let actual = parse_expr("x - - - x").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_compound1() {
        let expected = Expr::add(
            Expr::mul(Expr::Const(2), Expr::Var(String::from("x"))),
            Expr::Var(String::from("y")),
        );
        let actual = parse_expr("2 * x + y").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_compound2() {
        let expected = Expr::add(
            Expr::mul(
                Expr::add(Expr::mul(Expr::Const(0), Expr::Var(String::from("x"))), Expr::Const(1)),
                Expr::Const(3),
            ),
            Expr::Const(12),
        );
        let actual = parse_expr("(0 * x + 1) * 3 + 12").unwrap();
        assert_eq!(expected, actual);
    }
}
