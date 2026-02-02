use std::fmt;

use crate::syntax::{Expr, Op};

#[derive(Debug)]
pub enum Error {
    NegativePower,
    Metavar,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NegativePower => write!(f, "cannot raise to a negative power"),
            Error::Metavar => write!(f, "cannot simplify an expression containing a metavariable"),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

enum StackItem<'a> {
    Visit(&'a Expr),
    Eval(Op),
}

pub fn simplify(expr: &Expr) -> Result<Expr> {
    let mut stack = vec![StackItem::Visit(expr)];
    let mut results = Vec::new();

    while let Some(item) = stack.pop() {
        match item {
            StackItem::Eval(op) => {
                let result = if matches!(op, Op::Neg) {
                    let a = results.pop().unwrap();
                    simplify1(Expr::neg(a))?
                } else {
                    let b = results.pop().unwrap();
                    let a = results.pop().unwrap();
                    let expr = match op {
                        Op::Add => Expr::add(a, b),
                        Op::Sub => Expr::sub(a, b),
                        Op::Mul => Expr::mul(a, b),
                        Op::Exp => Expr::exp(a, b),
                        Op::Neg => unreachable!(),
                    };
                    simplify1(expr)?
                };
                results.push(result);
            }
            StackItem::Visit(expr) => match expr {
                leaf @ (Expr::Var(_) | Expr::Const(_)) => {
                    results.push(leaf.clone());
                }
                Expr::Neg(a) => {
                    stack.push(StackItem::Eval(Op::Neg));
                    stack.push(StackItem::Visit(a));
                }
                Expr::Add(a, b) | Expr::Sub(a, b) | Expr::Mul(a, b) | Expr::Exp(a, b) => {
                    let Some(op) = expr.op() else { unreachable!() };
                    stack.push(StackItem::Eval(op));
                    stack.push(StackItem::Visit(b));
                    stack.push(StackItem::Visit(a));
                }
                Expr::Metavar(_) => return Err(Error::Metavar),
            },
        }
    }

    let mut result = {
        assert_eq!(results.len(), 1);
        results.pop().unwrap()
    };

    loop {
        let simplified = simplify1(result.clone())?;
        if simplified == result {
            break Ok(result);
        }
        result = simplified;
    }
}

fn simplify1(expr: Expr) -> Result<Expr> {
    match expr {
        Expr::Add(a, b) => simplify1::add(*a, *b),
        Expr::Sub(a, b) => simplify1::sub(*a, *b),
        Expr::Mul(a, b) => simplify1::mul(*a, *b),
        Expr::Exp(a, b) => simplify1::exp(*a, *b),
        Expr::Neg(a) => simplify1::neg(*a),
        leaf @ (Expr::Const(_) | Expr::Var(_)) => Ok(leaf),
        Expr::Metavar(_) => Err(Error::Metavar),
    }
}

mod simplify1 {
    use crate::syntax::Expr;

    use super::{Error, Result};

    #[inline]
    pub fn add(a: Expr, b: Expr) -> Result<Expr> {
        match (a, b) {
            (Expr::Const(0), x) | (x, Expr::Const(0)) => Ok(x),
            (Expr::Const(m), Expr::Const(n)) => Ok(Expr::Const(m + n)),
            (Expr::Sub(e, c), Expr::Const(m)) if matches!(*c, Expr::Const(n) if n == m) => Ok(*e),
            (Expr::Const(m), Expr::Sub(e, c)) if matches!(*c, Expr::Const(n) if n == m) => Ok(*e),
            (Expr::Add(e, c), Expr::Const(m)) => {
                if let Expr::Const(n) = *c {
                    return Ok(Expr::add(e, Expr::Const(n + m)));
                }
                Ok(Expr::add(Expr::Add(e, c), Expr::Const(m)))
            }
            (Expr::Const(m), Expr::Add(e, c)) => {
                if let Expr::Const(n) = *c {
                    return Ok(Expr::add(e, Expr::Const(m + n)));
                }
                Ok(Expr::add(Expr::Const(m), Expr::Add(e, c)))
            }
            (a, b) => Ok(Expr::add(a, b)),
        }
    }

    #[inline]
    pub fn sub(a: Expr, b: Expr) -> Result<Expr> {
        match (a, b) {
            (x, Expr::Const(0)) => Ok(x),
            (Expr::Const(m), Expr::Const(n)) => Ok(Expr::Const(m - n)),
            (x, y) if x == y => Ok(Expr::Const(0)),
            (Expr::Add(e, c), Expr::Const(m)) if matches!(*c, Expr::Const(n) if n == m) => Ok(*e),
            (Expr::Const(m), Expr::Add(e, c)) if matches!(*c, Expr::Const(n) if n == m) => {
                Ok(Expr::neg(e))
            }
            (Expr::Sub(e, c), Expr::Const(n)) => {
                if let Expr::Const(m) = *c {
                    return Ok(Expr::sub(e, Expr::Const(m + n)));
                }
                Ok(Expr::sub(Expr::Sub(e, c), Expr::Const(n)))
            }
            (a, b) => Ok(Expr::sub(a, b)),
        }
    }

    #[inline]
    pub fn mul(a: Expr, b: Expr) -> Result<Expr> {
        match (a, b) {
            (Expr::Const(0), _) | (_, Expr::Const(0)) => Ok(Expr::Const(0)),
            (Expr::Const(1), x) | (x, Expr::Const(1)) => Ok(x),
            (Expr::Const(m), Expr::Const(n)) => Ok(Expr::Const(m * n)),
            (a, b) => Ok(Expr::mul(a, b)),
        }
    }

    #[inline]
    pub fn exp(a: Expr, b: Expr) -> Result<Expr> {
        match (a, b) {
            (_, Expr::Const(0)) => Ok(Expr::Const(1)),
            (Expr::Const(0), _) => Ok(Expr::Const(0)),
            (Expr::Const(1), _) => Ok(Expr::Const(1)),
            (x, Expr::Const(1)) => Ok(x),
            (_, Expr::Neg(n)) if matches!(*n, Expr::Const(_)) => Err(Error::NegativePower),
            (Expr::Const(m), Expr::Const(n)) => Ok(Expr::Const(m.pow(n as u32))),
            (a, b) => Ok(Expr::exp(a, b)),
        }
    }

    #[inline]
    pub fn neg(a: Expr) -> Result<Expr> {
        match a {
            Expr::Neg(x) => Ok(*x),
            Expr::Const(m) => Ok(Expr::Const(-m)),
            a => Ok(Expr::neg(a)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{parser, semantics};

    #[test]
    fn test_simplify_basic() {
        let expected = parser::parse_expr("42").unwrap();
        let input = parser::parse_expr("40 + 2").unwrap();
        let actual = semantics::simplify(&input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_simplify() {
        static INPUTS: &[(&str, &str)] = &[
            ("7", "1 + 2 * 3"),
            ("21", "(1 + 2) * (3 + 4)"),
            ("15", "(0 * x + 1) * 3 + 12"),
            ("0", "0 + (0 + (1 - 1))"),
            ("x + 15", "x + 15 - 12 * 0"),
            ("-x", "-(-(-(x)))"),
            ("x  + y", "0 + (x + (0 + y))"),
            ("x * y", "1 * (x * (1 * y))"),
            ("0", "z * (0 * (x * y))"),
            ("x - (y - (y - x))", "x - (y - (y - x))"),
            ("8", "2 ^ (1 + 2)"),
            ("x + 1", "(x + 0) * (1 + (y - y)) + (z ^ 0)"),
            ("x + z", "(x + 0) * (1 + (y - y)) + (z ^ 1)"),
            ("x + 3", "((((x + 1) - 1) + 2) - 2) + 3"),
            ("x", "5 + (x - 5)"),
            ("y + 3", "7 + ((y + 3) - 7)"),
            ("-z", "4 - (z + 4)"),
            ("-(a * b)", "10 - ((a * b) + 10)"),
            ("x", "3 + ((x - 1) - 2)"),
            ("-y", "5 - ((3 + (y + 2)))"),
            ("x * (y + z)", "x * (y + (z * (2 - 1))) + (0 * w)"),
            ("x * y", "(x * (y + 0)) + (0 * z)"),
            ("x * y", "x * (y ^ ((0 + 2) - 1))"),
            ("x", "((x * 1) + 0) - ((y - y) * z)"),
            ("1", "1 + ((x - x) * (y + z))"),
        ];
        for (expected_str, input_str) in INPUTS {
            let expected = parser::parse_expr(expected_str).unwrap();
            let input = parser::parse_expr(input_str).unwrap();
            let actual = semantics::simplify(&input).unwrap();
            assert_eq!(expected, actual, "{}", input_str);
        }
    }
}
