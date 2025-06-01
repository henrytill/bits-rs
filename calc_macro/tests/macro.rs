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
                let expected = parser::parse_expr($expected).unwrap();
                assert_eq!(expected, $input);
            }
        )*
    };
}

calc_tests! {
    test_basic => {
        input: calc!("2 * x + 1"),
        expected: "2 * x + 1"
    },
    test_antiquote => {
        input: {
            let x = parser::parse_expr("y + z").unwrap();
            calc!("2 * $x + 1")
        },
        expected: "2 * (y + z) + 1"
    },
    test_nested_antiquote => {
        input: {
            let x = calc!("y + z");
            calc!("2 * $x + 1")
        },
        expected: "2 * (y + z) + 1"
    },
    test_complex_expression => {
        input: calc!("2 * (x + 1) - 3 * (y - 4)"),
        expected: "2 * (x + 1) - 3 * (y - 4)"
    },
    test_deeply_nested_parens => {
        input: calc!("(((x + 1) * 2) + ((y - 3) * 4))"),
        expected: "(((x + 1) * 2) + ((y - 3) * 4))"
    },
    test_negation => {
        input: calc!("-x"),
        expected: "-x"
    },
    test_negation_with_parens => {
        input: calc!("-(x + y)"),
        expected: "-(x + y)"
    },
    test_double_negation => {
        input: calc!("--x"),
        expected: "--x"
    },
    test_exponentiation => {
        input: calc!("x ^ 2"),
        expected: "x ^ 2"
    },
    test_complex_exponentiation => {
        input: calc!("(x + 1) ^ (y - 2)"),
        expected: "(x + 1) ^ (y - 2)"
    },
    test_precedence => {
        input: calc!("x + y * z"),
        expected: "x + y * z"
    },
    test_exponentiation_precedence => {
        input: calc!("x * y ^ z"),
        expected: "x * y ^ z"
    },
    test_multiple_antiquotes => {
        input: {
            let x = calc!("a + b");
            let y = calc!("c * d");
            calc!("$x - $y")
        },
        expected: "(a + b) - (c * d)"
    },
    test_complex_with_multiple_antiquotes => {
        input: {
            let x = calc!("a + b");
            let y = calc!("c * d");
            calc!("($x ^ 2) - ($y + 3)")
        },
        expected: "((a + b) ^ 2) - ((c * d) + 3)"
    },
    test_single_constant => {
        input: calc!("42"),
        expected: "42"
    },
    test_single_variable => {
        input: calc!("variable"),
        expected: "variable"
    },
    test_single_antiquote => {
        input: {
            let expr = calc!("x * y");
            calc!("$expr")
        },
        expected: "x * y"
    },
    test_chained_operations => {
        input: calc!("a + b + c + d"),
        expected: "a + b + c + d"
    },
    test_mixed_chained_operations => {
        input: calc!("a + b * c - d"),
        expected: "a + b * c - d"
    },
    test_subtraction_double_negation => {
        input: {
            let x = parser::parse_expr("x").unwrap();
            let y = x.clone();
            calc!("$x - - - $y")
        },
        expected: "x - - - x"
    },
}
