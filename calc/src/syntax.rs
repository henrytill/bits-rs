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
            Expr::Metavar(x) => write!(f, "${}", x),
        }
    }
}

impl Expr {
    pub fn var(s: impl Into<String>) -> Expr {
        Expr::Var(s.into())
    }

    pub fn neg(e: impl Into<Box<Expr>>) -> Expr {
        Expr::Neg(e.into())
    }

    pub fn add(a: impl Into<Box<Expr>>, b: impl Into<Box<Expr>>) -> Expr {
        Expr::Add(a.into(), b.into())
    }

    pub fn sub(a: impl Into<Box<Expr>>, b: impl Into<Box<Expr>>) -> Expr {
        Expr::Sub(a.into(), b.into())
    }

    pub fn mul(a: impl Into<Box<Expr>>, b: impl Into<Box<Expr>>) -> Expr {
        Expr::Mul(a.into(), b.into())
    }

    pub fn exp(a: impl Into<Box<Expr>>, b: impl Into<Box<Expr>>) -> Expr {
        Expr::Exp(a.into(), b.into())
    }

    pub fn metavar(s: impl Into<String>) -> Expr {
        Expr::Metavar(s.into())
    }
}
