use calc::syntax::*;
use calc_macro::calc;

#[test]
fn test_basic_expr() {
    let actual = calc!("2 * x + 1");
    let expected = Expr::Add(
        Box::new(Expr::Mul(Box::new(Expr::Const(2)), Box::new(Expr::Var("x".to_string())))),
        Box::new(Expr::Const(1)),
    );
    assert_eq!(expected, actual);
}

#[test]
fn test_antiquote() {
    let x = Expr::Add(Box::new(Expr::Var("y".to_string())), Box::new(Expr::Var("z".to_string())));
    let actual = calc!("2 * $x + 1");
    let expected = Expr::Add(
        Box::new(Expr::Mul(
            Box::new(Expr::Const(2)),
            Box::new(Expr::Add(
                Box::new(Expr::Var("y".to_string())),
                Box::new(Expr::Var("z".to_string())),
            )),
        )),
        Box::new(Expr::Const(1)),
    );
    assert_eq!(expected, actual);
}

#[test]
fn test_nested_antiquote() {
    let x = calc!("y + z");
    let actual = calc!("2 * $x + 1");
    let expected = Expr::Add(
        Box::new(Expr::Mul(
            Box::new(Expr::Const(2)),
            Box::new(Expr::Add(
                Box::new(Expr::Var("y".to_string())),
                Box::new(Expr::Var("z".to_string())),
            )),
        )),
        Box::new(Expr::Const(1)),
    );
    assert_eq!(expected, actual);
}

#[test]
fn test_complex_expression() {
    let actual = calc!("2 * (x + 1) - 3 * (y - 4)");
    let expected = Expr::Sub(
        Box::new(Expr::Mul(
            Box::new(Expr::Const(2)),
            Box::new(Expr::Add(Box::new(Expr::Var("x".to_string())), Box::new(Expr::Const(1)))),
        )),
        Box::new(Expr::Mul(
            Box::new(Expr::Const(3)),
            Box::new(Expr::Sub(Box::new(Expr::Var("y".to_string())), Box::new(Expr::Const(4)))),
        )),
    );
    assert_eq!(expected, actual);
}

#[test]
fn test_deeply_nested_parens() {
    let actual = calc!("(((x + 1) * 2) + ((y - 3) * 4))");
    let expected = Expr::Add(
        Box::new(Expr::Mul(
            Box::new(Expr::Add(Box::new(Expr::Var("x".to_string())), Box::new(Expr::Const(1)))),
            Box::new(Expr::Const(2)),
        )),
        Box::new(Expr::Mul(
            Box::new(Expr::Sub(Box::new(Expr::Var("y".to_string())), Box::new(Expr::Const(3)))),
            Box::new(Expr::Const(4)),
        )),
    );
    assert_eq!(expected, actual);
}

#[test]
fn test_negation() {
    let actual = calc!("-x");
    let expected = Expr::Neg(Box::new(Expr::Var("x".to_string())));
    assert_eq!(expected, actual);
}

#[test]
fn test_negation_with_parens() {
    let actual = calc!("-(x + y)");
    let expected = Expr::Neg(Box::new(Expr::Add(
        Box::new(Expr::Var("x".to_string())),
        Box::new(Expr::Var("y".to_string())),
    )));
    assert_eq!(expected, actual);
}

#[test]
fn test_double_negation() {
    let actual = calc!("--x");
    let expected = Expr::Neg(Box::new(Expr::Neg(Box::new(Expr::Var("x".to_string())))));
    assert_eq!(expected, actual);
}

#[test]
fn test_exponentiation() {
    let actual = calc!("x ^ 2");
    let expected = Expr::Exp(Box::new(Expr::Var("x".to_string())), Box::new(Expr::Const(2)));
    assert_eq!(expected, actual);
}

#[test]
fn test_complex_exponentiation() {
    let actual = calc!("(x + 1) ^ (y - 2)");
    let expected = Expr::Exp(
        Box::new(Expr::Add(Box::new(Expr::Var("x".to_string())), Box::new(Expr::Const(1)))),
        Box::new(Expr::Sub(Box::new(Expr::Var("y".to_string())), Box::new(Expr::Const(2)))),
    );
    assert_eq!(expected, actual);
}

#[test]
fn test_precedence() {
    let actual = calc!("x + y * z");
    let expected = Expr::Add(
        Box::new(Expr::Var("x".to_string())),
        Box::new(Expr::Mul(
            Box::new(Expr::Var("y".to_string())),
            Box::new(Expr::Var("z".to_string())),
        )),
    );
    assert_eq!(expected, actual);
}

#[test]
fn test_exponentiation_precedence() {
    let actual = calc!("x * y ^ z");
    let expected = Expr::Mul(
        Box::new(Expr::Var("x".to_string())),
        Box::new(Expr::Exp(
            Box::new(Expr::Var("y".to_string())),
            Box::new(Expr::Var("z".to_string())),
        )),
    );
    assert_eq!(expected, actual);
}

#[test]
fn test_multiple_antiquotes() {
    let x = calc!("a + b");
    let y = calc!("c * d");
    let actual = calc!("$x - $y");
    let expected = Expr::Sub(
        Box::new(Expr::Add(
            Box::new(Expr::Var("a".to_string())),
            Box::new(Expr::Var("b".to_string())),
        )),
        Box::new(Expr::Mul(
            Box::new(Expr::Var("c".to_string())),
            Box::new(Expr::Var("d".to_string())),
        )),
    );
    assert_eq!(expected, actual);
}

#[test]
fn test_complex_with_multiple_antiquotes() {
    let x = calc!("a + b");
    let y = calc!("c * d");
    let actual = calc!("($x ^ 2) - ($y + 3)");
    let expected = Expr::Sub(
        Box::new(Expr::Exp(
            Box::new(Expr::Add(
                Box::new(Expr::Var("a".to_string())),
                Box::new(Expr::Var("b".to_string())),
            )),
            Box::new(Expr::Const(2)),
        )),
        Box::new(Expr::Add(
            Box::new(Expr::Mul(
                Box::new(Expr::Var("c".to_string())),
                Box::new(Expr::Var("d".to_string())),
            )),
            Box::new(Expr::Const(3)),
        )),
    );
    assert_eq!(expected, actual);
}

#[test]
fn test_single_constant() {
    let actual = calc!("42");
    let expected = Expr::Const(42);
    assert_eq!(expected, actual);
}

#[test]
fn test_single_variable() {
    let actual = calc!("variable");
    let expected = Expr::Var("variable".to_string());
    assert_eq!(expected, actual);
}

#[test]
fn test_single_antiquote() {
    let expr = calc!("x * y");
    let actual = calc!("$expr");
    let expected =
        Expr::Mul(Box::new(Expr::Var("x".to_string())), Box::new(Expr::Var("y".to_string())));
    assert_eq!(expected, actual);
}

#[test]
fn test_chained_operations() {
    let actual = calc!("a + b + c + d");
    // Due to left associativity, this should be ((a + b) + c) + d
    let expected = Expr::Add(
        Box::new(Expr::Add(
            Box::new(Expr::Add(
                Box::new(Expr::Var("a".to_string())),
                Box::new(Expr::Var("b".to_string())),
            )),
            Box::new(Expr::Var("c".to_string())),
        )),
        Box::new(Expr::Var("d".to_string())),
    );
    assert_eq!(expected, actual);
}

#[test]
fn test_mixed_chained_operations() {
    let actual = calc!("a + b * c - d");
    let expected = Expr::Sub(
        Box::new(Expr::Add(
            Box::new(Expr::Var("a".to_string())),
            Box::new(Expr::Mul(
                Box::new(Expr::Var("b".to_string())),
                Box::new(Expr::Var("c".to_string())),
            )),
        )),
        Box::new(Expr::Var("d".to_string())),
    );
    assert_eq!(expected, actual);
}
