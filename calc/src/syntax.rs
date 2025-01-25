use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Var(String),
    Const(i32),
    Neg(Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Exp(Box<Expr>, Box<Expr>),
    Metavar(String),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Var(x) => write!(f, "{}", x),
            Expr::Const(n) => write!(f, "{}", n),
            Expr::Neg(e) => write!(f, "(-{})", e),
            Expr::Add(a, b) => write!(f, "({} + {})", a, b),
            Expr::Sub(a, b) => write!(f, "({} - {})", a, b),
            Expr::Mul(a, b) => write!(f, "({} * {})", a, b),
            Expr::Exp(a, b) => write!(f, "({} ^ {})", a, b),
            Expr::Metavar(s) => write!(f, "${}", s),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    NegativePower,
    Metavar,
    Parse(String),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NegativePower => write!(f, "cannot raise to a negative power"),
            Error::Metavar => write!(f, "metavariable"),
            Error::Parse(msg) => write!(f, "parse error: {}", msg),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

fn pow(a: i32, n: i32) -> Result<i32> {
    match n {
        0 => Ok(1),
        1 => Ok(a),
        n if n < 0 => Err(Error::NegativePower),
        n => {
            let b = pow(a, n / 2)?;
            Ok(b * b * if n % 2 == 0 { 1 } else { a })
        }
    }
}

pub fn simplify1(expr: &Expr) -> Result<Expr> {
    match expr {
        Expr::Add(a, b) => match (&**a, &**b) {
            (Expr::Const(0), x) | (x, Expr::Const(0)) => Ok(x.clone()),
            (Expr::Const(m), Expr::Const(n)) => Ok(Expr::Const(m + n)),
            _ => Ok(expr.clone()),
        },
        Expr::Sub(a, b) => match (&**a, &**b) {
            (x, Expr::Const(0)) => Ok(x.clone()),
            (x, y) if x == y => Ok(Expr::Const(0)),
            (Expr::Const(m), Expr::Const(n)) => Ok(Expr::Const(m - n)),
            _ => Ok(expr.clone()),
        },
        Expr::Mul(a, b) => match (&**a, &**b) {
            (Expr::Const(0), _) | (_, Expr::Const(0)) => Ok(Expr::Const(0)),
            (Expr::Const(1), x) | (x, Expr::Const(1)) => Ok(x.clone()),
            (Expr::Const(m), Expr::Const(n)) => Ok(Expr::Const(m * n)),
            _ => Ok(expr.clone()),
        },
        Expr::Exp(a, b) => match (&**a, &**b) {
            (_, Expr::Const(0)) => Ok(Expr::Const(1)),
            (Expr::Const(0), _) => Ok(Expr::Const(0)),
            (Expr::Const(1), _) => Ok(Expr::Const(1)),
            (x, Expr::Const(1)) => Ok(x.clone()),
            (_, Expr::Neg(n)) if matches!(&**n, Expr::Const(_)) => Err(Error::NegativePower),
            (Expr::Const(m), Expr::Const(n)) => Ok(Expr::Const(pow(*m, *n)?)),
            _ => Ok(expr.clone()),
        },
        Expr::Neg(a) => match &**a {
            Expr::Neg(x) => Ok(x.as_ref().clone()),
            Expr::Const(m) => Ok(Expr::Const(-m)),
            _ => Ok(expr.clone()),
        },
        Expr::Metavar(_) => Err(Error::Metavar),
        _ => Ok(expr.clone()),
    }
}

pub fn simplify(expr: &Expr) -> Result<Expr> {
    let mut current = expr.clone();
    loop {
        let simplified = match &current {
            Expr::Add(a, b) => {
                let a = simplify(a)?;
                let b = simplify(b)?;
                simplify1(&Expr::Add(Box::new(a), Box::new(b)))?
            }
            Expr::Sub(a, b) => {
                let a = simplify(a)?;
                let b = simplify(b)?;
                simplify1(&Expr::Sub(Box::new(a), Box::new(b)))?
            }
            Expr::Mul(a, b) => {
                let a = simplify(a)?;
                let b = simplify(b)?;
                simplify1(&Expr::Mul(Box::new(a), Box::new(b)))?
            }
            Expr::Exp(a, b) => {
                let a = simplify(a)?;
                let b = simplify(b)?;
                simplify1(&Expr::Exp(Box::new(a), Box::new(b)))?
            }
            Expr::Neg(a) => {
                let a = simplify(a)?;
                simplify1(&Expr::Neg(Box::new(a)))?
            }
            _ => simplify1(&current)?,
        };

        if simplified == current {
            break Ok(current);
        }
        current = simplified;
    }
}
