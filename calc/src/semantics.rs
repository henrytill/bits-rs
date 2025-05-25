use std::fmt;

use crate::syntax::Expr;

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
            Error::Metavar => write!(f, "metavariable"),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq, Eq)]
struct StackItem<'a> {
    expr: &'a Expr,
    visited: bool,
}

// Simplify using post-order traversal
pub fn simplify(expr: &Expr) -> Result<Expr> {
    let mut stack = vec![StackItem { expr, visited: false }];
    let mut results = Vec::new();

    while let Some(item) = stack.pop() {
        if item.visited {
            // Process this node using already simplified children
            let result = match item.expr {
                Expr::Add(_, _) => {
                    let b_res = results.pop().unwrap();
                    let a_res = results.pop().unwrap();
                    simplify1(Expr::Add(Box::new(a_res), Box::new(b_res)))?
                }
                Expr::Sub(_, _) => {
                    let b_res = results.pop().unwrap();
                    let a_res = results.pop().unwrap();
                    simplify1(Expr::Sub(Box::new(a_res), Box::new(b_res)))?
                }
                Expr::Mul(_, _) => {
                    let b_res = results.pop().unwrap();
                    let a_res = results.pop().unwrap();
                    simplify1(Expr::Mul(Box::new(a_res), Box::new(b_res)))?
                }
                Expr::Exp(_, _) => {
                    let b_res = results.pop().unwrap();
                    let a_res = results.pop().unwrap();
                    simplify1(Expr::Exp(Box::new(a_res), Box::new(b_res)))?
                }
                Expr::Neg(_) => {
                    let a_res = results.pop().unwrap();
                    simplify1(Expr::Neg(Box::new(a_res)))?
                }
                expr => simplify1(expr.clone())?,
            };
            results.push(result);
        } else {
            match item.expr {
                Expr::Add(a, b) | Expr::Sub(a, b) | Expr::Mul(a, b) | Expr::Exp(a, b) => {
                    stack.push(StackItem { expr: item.expr, visited: true });
                    stack.push(StackItem { expr: b.as_ref(), visited: false });
                    stack.push(StackItem { expr: a.as_ref(), visited: false });
                }
                Expr::Neg(a) => {
                    stack.push(StackItem { expr: item.expr, visited: true });
                    stack.push(StackItem { expr: a.as_ref(), visited: false });
                }
                expr => {
                    // For leaf nodes, just simplify directly
                    let result = simplify1(expr.clone())?;
                    results.push(result);
                }
            }
        }
    }

    // The final result should be the only item in the results vector
    assert_eq!(results.len(), 1);

    let mut result = results.pop().unwrap();

    // Keep simplifying until we reach a fixed point
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
        Expr::Metavar(_) => Err(Error::Metavar),
        expr => Ok(expr),
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
            // Handle (x - c1) + c2 -> x when c1 == c2
            (Expr::Sub(e, c), Expr::Const(m)) => {
                if let Expr::Const(n) = *c {
                    if n == m {
                        return Ok(*e);
                    }
                }
                Ok(Expr::add(Expr::Sub(e, c), Expr::Const(m)))
            }
            // Handle c1 + (x - c2) -> x when c1 == c2
            (Expr::Const(m), Expr::Sub(e, c)) => {
                if let Expr::Const(n) = *c {
                    if n == m {
                        return Ok(*e);
                    }
                }
                Ok(Expr::add(Expr::Const(m), Expr::Sub(e, c)))
            }
            // Handle (e + c1) + c2 -> e + (c1 + c2)
            (Expr::Add(e, c), Expr::Const(m)) => {
                if let Expr::Const(n) = *c {
                    return Ok(Expr::add(e, Expr::Const(n + m)));
                }
                Ok(Expr::add(Expr::Add(e, c), Expr::Const(m)))
            }
            // Handle c1 + (e + c2) -> e + (c1 + c2)
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
            // Handle (x + c1) - c2 -> x when c1 == c2
            (Expr::Add(e, c), Expr::Const(m)) => {
                if let Expr::Const(n) = *c {
                    if n == m {
                        return Ok(*e);
                    }
                }
                Ok(Expr::sub(Expr::Add(e, c), Expr::Const(m)))
            }
            // Handle c1 - (x + c2) -> -x when c1 == c2
            (Expr::Const(m), Expr::Add(e, c)) => {
                if let Expr::Const(n) = *c {
                    if n == m {
                        return Ok(Expr::neg(e));
                    }
                }
                Ok(Expr::sub(Expr::Const(m), Expr::Add(e, c)))
            }
            // Handle (e - c1) - c2 -> e - (c1 + c2)
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
            // Tests for c1 + (x - c2) -> x when c1 == c2
            ("x", "5 + (x - 5)"),
            ("y + 3", "7 + ((y + 3) - 7)"),
            // Tests for c1 - (x + c2) -> -x when c1 == c2
            ("-z", "4 - (z + 4)"),
            ("-(a * b)", "10 - ((a * b) + 10)"),
            // More complex nested cases
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
