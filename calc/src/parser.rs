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

    macro_rules! parser_tests {
        (
            $(
                $test_name:ident => {
                    input: $input:expr,
                    expected: $expected:expr
                }
            ),* $(,)?
        ) => {
            $(
                #[test]
                fn $test_name() {
                    let actual = parse_expr($input).unwrap();
                    assert_eq!($expected, actual);
                }
            )*
        };
    }

    parser_tests! {
        test_parse_const => {
            input: "42",
            expected: Expr::Const(42)
        },
        test_parse_var => {
            input: "x",
            expected: Expr::var("x")
        },
        test_parse_ops => {
            input: "1 + 2 * 3",
            expected: Expr::add(Expr::Const(1), Expr::mul(Expr::Const(2), Expr::Const(3)))
        },
        test_parse_precedence => {
            input: "1 * 2 + 3",
            expected: Expr::add(Expr::mul(Expr::Const(1), Expr::Const(2)), Expr::Const(3))
        },
        test_parse_sub_double_negation => {
            input: "x - - - x",
            expected: Expr::sub(
                Expr::var("x"),
                Expr::neg(Expr::neg(Expr::var("x"))),
            )
        },
        test_parse_compound1 => {
            input: "2 * x + y",
            expected: Expr::add(
                Expr::mul(Expr::Const(2), Expr::var("x")),
                Expr::var("y"),
            )
        },
        test_parse_compound2 => {
            input: "(0 * x + 1) * 3 + 12",
            expected: Expr::add(
                Expr::mul(
                    Expr::add(
                        Expr::mul(Expr::Const(0), Expr::var("x")),
                        Expr::Const(1),
                    ),
                    Expr::Const(3),
                ),
                Expr::Const(12),
            )
        },
        test_parse_metavar => {
            input: "$x",
            expected: Expr::metavar("x")
        },
        test_parse_exp => {
            input: "2 ^ 3",
            expected: Expr::exp(Expr::Const(2), Expr::Const(3))
        },
        test_parse_exp_with_var => {
            input: "x ^ 2",
            expected: Expr::exp(Expr::var("x"), Expr::Const(2))
        },
        test_parse_exp_with_negation => {
            input: "-x ^ 2",
            expected: Expr::exp(Expr::neg(Expr::var("x")), Expr::Const(2))
        },
        test_parse_exp_with_addition => {
            input: "x + y ^ 2",
            expected: Expr::add(
                Expr::var("x"),
                Expr::exp(Expr::var("y"), Expr::Const(2)),
            )
        },
        test_parse_exp_with_mixed_ops => {
            input: "x + y ^ 2 * z",
            expected: Expr::add(
                Expr::var("x"),
                Expr::mul(
                    Expr::exp(Expr::var("y"), Expr::Const(2)),
                    Expr::var("z"),
                ),
            )
        },
    }
}
