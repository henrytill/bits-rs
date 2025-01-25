use calc::syntax::*;
use calc_macro::calc;

#[test]
fn test_basic_expr() {
    let actual = calc!("2 * x + 1");
    let expected = Expr::Add(
        Box::new(Expr::Mul(
            Box::new(Expr::Const(2)),
            Box::new(Expr::Var("x".to_string())),
        )),
        Box::new(Expr::Const(1)),
    );
    assert_eq!(expected, actual);
}

#[test]
fn test_antiquote() {
    let x = Expr::Add(
        Box::new(Expr::Var("y".to_string())),
        Box::new(Expr::Var("z".to_string())),
    );
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
