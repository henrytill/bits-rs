use calc::parser;
use calc_macro::calc;

macro_rules! calc_tests {
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
                let actual = parser::parse_expr($input).unwrap();
                assert_eq!($expected, actual);
            }
        )*
    };
}

calc_tests! {
    test_basic => {
        input: "2 * x + 1",
        expected: calc!("2 * x + 1")
    },
    test_antiquote => {
        input: "2 * (y + z) + 1",
        expected: {
            let x = parser::parse_expr("y + z").unwrap();
            calc!("2 * $x + 1")
        }
    },
    test_nested_antiquote => {
        input: "2 * (y + z) + 1",
        expected: {
            let x = calc!("y + z");
            calc!("2 * $x + 1")
        }
    },
    test_complex_expression => {
        input: "2 * (x + 1) - 3 * (y - 4)",
        expected: calc!("2 * (x + 1) - 3 * (y - 4)")
    },
    test_deeply_nested_parens => {
        input: "(((x + 1) * 2) + ((y - 3) * 4))",
        expected: calc!("(((x + 1) * 2) + ((y - 3) * 4))")
    },
    test_negation => {
        input: "-x",
        expected: calc!("-x")
    },
    test_negation_with_parens => {
        input: "-(x + y)",
        expected: calc!("-(x + y)")
    },
    test_double_negation => {
        input: "--x",
        expected: calc!("--x")
    },
    test_exponentiation => {
        input: "x ^ 2",
        expected: calc!("x ^ 2")
    },
    test_complex_exponentiation => {
        input: "(x + 1) ^ (y - 2)",
        expected: calc!("(x + 1) ^ (y - 2)")
    },
    test_precedence => {
        input: "x + y * z",
        expected: calc!("x + y * z")
    },
    test_exponentiation_precedence => {
        input: "x * y ^ z",
        expected: calc!("x * y ^ z")
    },
    test_multiple_antiquotes => {
        input: "(a + b) - (c * d)",
        expected: {
            let x = calc!("a + b");
            let y = calc!("c * d");
            calc!("$x - $y")
        }
    },
    test_complex_with_multiple_antiquotes => {
        input: "((a + b) ^ 2) - ((c * d) + 3)",
        expected: {
            let x = calc!("a + b");
            let y = calc!("c * d");
            calc!("($x ^ 2) - ($y + 3)")
        }
    },
    test_single_constant => {
        input: "42",
        expected: calc!("42")
    },
    test_single_variable => {
        input: "variable",
        expected: calc!("variable")
    },
    test_single_antiquote => {
        input: "x * y",
        expected: {
            let expr = calc!("x * y");
            calc!("$expr")
        }
    },
    test_chained_operations => {
        input: "a + b + c + d",
        expected: calc!("a + b + c + d")
    },
    test_mixed_chained_operations => {
        input: "a + b * c - d",
        expected: calc!("a + b * c - d")
    },
    test_subtraction_double_negation => {
        input: "x - - - x",
        expected: {
            let x = parser::parse_expr("x").unwrap();
            let y = x.clone();
            calc!("$x - - - $y")
        }
    },
}
