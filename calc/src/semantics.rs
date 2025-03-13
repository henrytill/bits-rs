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

fn pow(a: i32, n: i32) -> Result<i32> {
    match n {
        0 => Ok(1),
        1 => Ok(a),
        n if n < 0 => Err(Error::NegativePower),
        n => Ok(a.pow(n as u32)),
    }
}

pub fn simplify1(expr: Expr) -> Result<Expr> {
    match expr {
        Expr::Add(a, b) => match (*a, *b) {
            (Expr::Const(0), x) | (x, Expr::Const(0)) => Ok(x),
            (Expr::Const(m), Expr::Const(n)) => Ok(Expr::Const(m + n)),
            // Handle (x - c1) + c2 -> x when c1 == c2
            (Expr::Sub(e, c), Expr::Const(m)) => {
                if let Expr::Const(n) = *c {
                    if n == m {
                        return Ok(*e);
                    }
                }
                Ok(Expr::Add(Box::new(Expr::Sub(e, c)), Box::new(Expr::Const(m))))
            }
            // Handle c1 + (x - c2) -> x when c1 == c2
            (Expr::Const(m), Expr::Sub(e, c)) => {
                if let Expr::Const(n) = *c {
                    if n == m {
                        return Ok(*e);
                    }
                }
                Ok(Expr::Add(Box::new(Expr::Const(m)), Box::new(Expr::Sub(e, c))))
            }
            // Handle c1 + (e + c2) -> e + (c1 + c2)
            (Expr::Const(m), Expr::Add(e, c)) => {
                if let Expr::Const(n) = *c {
                    return Ok(Expr::Add(e, Box::new(Expr::Const(m + n))));
                }
                Ok(Expr::Add(Box::new(Expr::Const(m)), Box::new(Expr::Add(e, c))))
            }
            // Handle (e + c1) + c2 -> e + (c1 + c2)
            (Expr::Add(e, c), Expr::Const(m)) => {
                if let Expr::Const(n) = *c {
                    return Ok(Expr::Add(e, Box::new(Expr::Const(n + m))));
                }
                Ok(Expr::Add(Box::new(Expr::Add(e, c)), Box::new(Expr::Const(m))))
            }
            (a, b) => Ok(Expr::Add(Box::new(a), Box::new(b))),
        },
        Expr::Sub(a, b) => match (*a, *b) {
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
                Ok(Expr::Sub(Box::new(Expr::Add(e, c)), Box::new(Expr::Const(m))))
            }
            // Handle c1 - (x + c2) -> -x when c1 == c2
            (Expr::Const(m), Expr::Add(e, c)) => {
                if let Expr::Const(n) = *c {
                    if n == m {
                        return Ok(Expr::Neg(Box::new(*e)));
                    }
                }
                Ok(Expr::Sub(Box::new(Expr::Const(m)), Box::new(Expr::Add(e, c))))
            }
            // Handle (e - c1) - c2 -> e - (c1 + c2)
            (Expr::Sub(e, c), Expr::Const(n)) => {
                if let Expr::Const(m) = *c {
                    return Ok(Expr::Sub(e, Box::new(Expr::Const(m + n))));
                }
                Ok(Expr::Sub(Box::new(Expr::Sub(e, c)), Box::new(Expr::Const(n))))
            }
            (a, b) => Ok(Expr::Sub(Box::new(a), Box::new(b))),
        },
        Expr::Mul(a, b) => match (*a, *b) {
            (Expr::Const(0), _) | (_, Expr::Const(0)) => Ok(Expr::Const(0)),
            (Expr::Const(1), x) | (x, Expr::Const(1)) => Ok(x),
            (Expr::Const(m), Expr::Const(n)) => Ok(Expr::Const(m * n)),
            (a, b) => Ok(Expr::Mul(Box::new(a), Box::new(b))),
        },
        Expr::Exp(a, b) => match (*a, *b) {
            (_, Expr::Const(0)) => Ok(Expr::Const(1)),
            (Expr::Const(0), _) => Ok(Expr::Const(0)),
            (Expr::Const(1), _) => Ok(Expr::Const(1)),
            (x, Expr::Const(1)) => Ok(x),
            (_, Expr::Neg(n)) if matches!(*n, Expr::Const(_)) => Err(Error::NegativePower),
            (Expr::Const(m), Expr::Const(n)) => Ok(Expr::Const(pow(m, n)?)),
            (a, b) => Ok(Expr::Exp(Box::new(a), Box::new(b))),
        },
        Expr::Neg(a) => match *a {
            Expr::Neg(x) => Ok(*x),
            Expr::Const(m) => Ok(Expr::Const(-m)),
            a => Ok(Expr::Neg(Box::new(a))),
        },
        Expr::Metavar(_) => Err(Error::Metavar),
        expr => Ok(expr),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct StackItem {
    expr: Expr,
    visited: bool,
}

// Simplify using post-order traversal
pub fn simplify(expr: Expr) -> Result<Expr> {
    let mut stack = vec![StackItem { expr, visited: false }];

    let mut results = Vec::new();

    while let Some(mut item) = stack.pop() {
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
                expr => simplify1(expr)?,
            };
            results.push(result);
        } else {
            // Mark as visited and add item (parent) and its children to stack
            item.visited = true;
            let parent = item.clone();

            match item.expr {
                Expr::Add(a, b) | Expr::Sub(a, b) | Expr::Mul(a, b) | Expr::Exp(a, b) => {
                    stack.push(parent);
                    stack.push(StackItem { expr: *b, visited: false });
                    stack.push(StackItem { expr: *a, visited: false });
                }
                Expr::Neg(a) => {
                    stack.push(parent);
                    stack.push(StackItem { expr: *a, visited: false });
                }
                expr => {
                    // For leaf nodes, just simplify directly
                    let result = simplify1(expr)?;
                    results.push(result);
                }
            }
        }
    }

    // The final result should be the only item in the results vector
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

#[cfg(test)]
mod tests {
    use crate::syntax::Expr;
    use crate::{parser, semantics};

    #[test]
    fn test_simplify_basic() {
        let input = parser::parse_expr("40 + 2").unwrap();
        let actual = semantics::simplify(input).unwrap();
        let expected = Expr::Const(42);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_simplify() {
        let inputs: &[(Expr, &str)] = &[
            (Expr::Const(7), "1 + 2 * 3"),
            (Expr::Const(21), "(1 + 2) * (3 + 4)"),
            (Expr::Const(15), "(0 * x + 1) * 3 + 12"),
            (Expr::Const(0), "0 + (0 + (1 - 1))"),
            (
                Expr::Add(Box::new(Expr::Var(String::from("x"))), Box::new(Expr::Const(15))),
                "x + 15 - 12 * 0",
            ),
            (Expr::Neg(Box::new(Expr::Var(String::from("x")))), "-(-(-(x)))"),
            (
                Expr::Add(
                    Box::new(Expr::Var(String::from("x"))),
                    Box::new(Expr::Var(String::from("y"))),
                ),
                "0 + (x + (0 + y))",
            ),
            (
                Expr::Mul(
                    Box::new(Expr::Var(String::from("x"))),
                    Box::new(Expr::Var(String::from("y"))),
                ),
                "1 * (x * (1 * y))",
            ),
            (Expr::Const(0), "z * (0 * (x * y))"),
            (
                Expr::Sub(
                    Box::new(Expr::Var(String::from("x"))),
                    Box::new(Expr::Sub(
                        Box::new(Expr::Var(String::from("y"))),
                        Box::new(Expr::Sub(
                            Box::new(Expr::Var(String::from("y"))),
                            Box::new(Expr::Var(String::from("x"))),
                        )),
                    )),
                ),
                "x - (y - (y - x))",
            ),
            (Expr::Const(8), "2 ^ (1 + 2)"),
            (
                Expr::Add(Box::new(Expr::Var(String::from("x"))), Box::new(Expr::Const(1))),
                "(x + 0) * (1 + (y - y)) + (z ^ 0)",
            ),
            (
                Expr::Add(
                    Box::new(Expr::Var(String::from("x"))),
                    Box::new(Expr::Var(String::from("z"))),
                ),
                "(x + 0) * (1 + (y - y)) + (z ^ 1)",
            ),
            (
                Expr::Add(Box::new(Expr::Var(String::from("x"))), Box::new(Expr::Const(3))),
                "((((x + 1) - 1) + 2) - 2) + 3",
            ),
            // Tests for c1 + (x - c2) -> x when c1 == c2
            (Expr::Var(String::from("x")), "5 + (x - 5)"),
            (
                Expr::Add(Box::new(Expr::Var(String::from("y"))), Box::new(Expr::Const(3))),
                "7 + ((y + 3) - 7)",
            ),
            // Tests for c1 - (x + c2) -> -x when c1 == c2
            (Expr::Neg(Box::new(Expr::Var(String::from("z")))), "4 - (z + 4)"),
            (
                Expr::Neg(Box::new(Expr::Mul(
                    Box::new(Expr::Var(String::from("a"))),
                    Box::new(Expr::Var(String::from("b"))),
                ))),
                "10 - ((a * b) + 10)",
            ),
            // More complex nested cases
            (Expr::Var(String::from("x")), "3 + ((x - 1) - 2)"),
            (Expr::Neg(Box::new(Expr::Var(String::from("y")))), "5 - ((3 + (y + 2)))"),
            (
                Expr::Mul(
                    Box::new(Expr::Var(String::from("x"))),
                    Box::new(Expr::Add(
                        Box::new(Expr::Var(String::from("y"))),
                        Box::new(Expr::Var(String::from("z"))),
                    )),
                ),
                "x * (y + (z * (2 - 1))) + (0 * w)",
            ),
            (
                Expr::Mul(
                    Box::new(Expr::Var(String::from("x"))),
                    Box::new(Expr::Var(String::from("y"))),
                ),
                "(x * (y + 0)) + (0 * z)",
            ),
            (
                Expr::Mul(
                    Box::new(Expr::Var(String::from("x"))),
                    Box::new(Expr::Var(String::from("y"))),
                ),
                "x * (y ^ ((0 + 2) - 1))",
            ),
            (Expr::Var(String::from("x")), "((x * 1) + 0) - ((y - y) * z)"),
            (Expr::Const(1), "1 + ((x - x) * (y + z))"),
        ];
        for (expected, input_str) in inputs {
            let input = parser::parse_expr(input_str).unwrap();
            let actual = semantics::simplify(input).unwrap();
            assert_eq!(*expected, actual, "{}", input_str);
        }
    }
}
