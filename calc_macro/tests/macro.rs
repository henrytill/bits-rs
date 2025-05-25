use calc::parser;
use calc_macro::calc;

#[test]
fn test_basic_expr() {
    let expected = parser::parse_expr("2 * x + 1").unwrap();
    let actual = calc!("2 * x + 1");
    assert_eq!(expected, actual);
}

#[test]
fn test_antiquote() {
    let expected = parser::parse_expr("2 * (y + z) + 1").unwrap();
    let x = parser::parse_expr("y + z").unwrap();
    let actual = calc!("2 * $x + 1");
    assert_eq!(expected, actual);
}

#[test]
fn test_nested_antiquote() {
    let expected = parser::parse_expr("2 * (y + z) + 1").unwrap();
    let x = calc!("y + z");
    let actual = calc!("2 * $x + 1");
    assert_eq!(expected, actual);
}

#[test]
fn test_complex_expression() {
    let expected = parser::parse_expr("2 * (x + 1) - 3 * (y - 4)").unwrap();
    let actual = calc!("2 * (x + 1) - 3 * (y - 4)");
    assert_eq!(expected, actual);
}

#[test]
fn test_deeply_nested_parens() {
    let expected = parser::parse_expr("(((x + 1) * 2) + ((y - 3) * 4))").unwrap();
    let actual = calc!("(((x + 1) * 2) + ((y - 3) * 4))");
    assert_eq!(expected, actual);
}

#[test]
fn test_negation() {
    let expected = parser::parse_expr("-x").unwrap();
    let actual = calc!("-x");
    assert_eq!(expected, actual);
}

#[test]
fn test_negation_with_parens() {
    let expected = parser::parse_expr("-(x + y)").unwrap();
    let actual = calc!("-(x + y)");
    assert_eq!(expected, actual);
}

#[test]
fn test_double_negation() {
    let expected = parser::parse_expr("--x").unwrap();
    let actual = calc!("--x");
    assert_eq!(expected, actual);
}

#[test]
fn test_exponentiation() {
    let expected = parser::parse_expr("x ^ 2").unwrap();
    let actual = calc!("x ^ 2");
    assert_eq!(expected, actual);
}

#[test]
fn test_complex_exponentiation() {
    let expected = parser::parse_expr("(x + 1) ^ (y - 2)").unwrap();
    let actual = calc!("(x + 1) ^ (y - 2)");
    assert_eq!(expected, actual);
}

#[test]
fn test_precedence() {
    let expected = parser::parse_expr("x + y * z").unwrap();
    let actual = calc!("x + y * z");
    assert_eq!(expected, actual);
}

#[test]
fn test_exponentiation_precedence() {
    let expected = parser::parse_expr("x * y ^ z").unwrap();
    let actual = calc!("x * y ^ z");
    assert_eq!(expected, actual);
}

#[test]
fn test_multiple_antiquotes() {
    let expected = parser::parse_expr("(a + b) - (c * d)").unwrap();
    let x = calc!("a + b");
    let y = calc!("c * d");
    let actual = calc!("$x - $y");
    assert_eq!(expected, actual);
}

#[test]
fn test_complex_with_multiple_antiquotes() {
    let expected = parser::parse_expr("((a + b) ^ 2) - ((c * d) + 3)").unwrap();
    let x = calc!("a + b");
    let y = calc!("c * d");
    let actual = calc!("($x ^ 2) - ($y + 3)");
    assert_eq!(expected, actual);
}

#[test]
fn test_single_constant() {
    let expected = parser::parse_expr("42").unwrap();
    let actual = calc!("42");
    assert_eq!(expected, actual);
}

#[test]
fn test_single_variable() {
    let expected = parser::parse_expr("variable").unwrap();
    let actual = calc!("variable");
    assert_eq!(expected, actual);
}

#[test]
fn test_single_antiquote() {
    let expected = parser::parse_expr("x * y").unwrap();
    let expr = calc!("x * y");
    let actual = calc!("$expr");
    assert_eq!(expected, actual);
}

#[test]
fn test_chained_operations() {
    let expected = parser::parse_expr("a + b + c + d").unwrap();
    let actual = calc!("a + b + c + d");
    assert_eq!(expected, actual);
}

#[test]
fn test_mixed_chained_operations() {
    let expected = parser::parse_expr("a + b * c - d").unwrap();
    let actual = calc!("a + b * c - d");
    assert_eq!(expected, actual);
}
