use proc_macro2::TokenStream;
use quote::quote;

use calc::syntax::Expr;

#[derive(Debug, PartialEq, Eq)]
struct StackItem<'a> {
    expr: &'a Expr,
    visited: bool,
}

pub fn expr_to_syntax(expr: &Expr) -> TokenStream {
    let mut stack = vec![StackItem { expr, visited: false }];
    let mut results = Vec::new();

    while let Some(item) = stack.pop() {
        if item.visited {
            let result = match item.expr {
                Expr::Neg(_) => {
                    let a = results.pop().unwrap();
                    quote! {
                        calc::syntax::Expr::Neg(Box::new(#a))
                    }
                }
                Expr::Add(_, _) => {
                    let b = results.pop().unwrap();
                    let a = results.pop().unwrap();
                    quote! {
                        calc::syntax::Expr::Add(Box::new(#a), Box::new(#b))
                    }
                }
                Expr::Sub(_, _) => {
                    let b = results.pop().unwrap();
                    let a = results.pop().unwrap();
                    quote! {
                        calc::syntax::Expr::Sub(Box::new(#a), Box::new(#b))
                    }
                }
                Expr::Mul(_, _) => {
                    let b = results.pop().unwrap();
                    let a = results.pop().unwrap();
                    quote! {
                        calc::syntax::Expr::Mul(Box::new(#a), Box::new(#b))
                    }
                }
                Expr::Exp(_, _) => {
                    let b = results.pop().unwrap();
                    let a = results.pop().unwrap();
                    quote! {
                        calc::syntax::Expr::Exp(Box::new(#a), Box::new(#b))
                    }
                }
                _ => panic!("We shouldn't be here"),
            };
            results.push(result);
        } else {
            match item.expr {
                Expr::Neg(a) => {
                    stack.push(StackItem { expr: item.expr, visited: true });
                    stack.push(StackItem { expr: a, visited: false });
                }
                Expr::Var(x) => {
                    results.push(quote! {
                        calc::syntax::Expr::Var(String::from(#x))
                    });
                }
                Expr::Const(n) => {
                    results.push(quote! {
                        calc::syntax::Expr::Const(#n)
                    });
                }
                Expr::Add(a, b) | Expr::Sub(a, b) | Expr::Mul(a, b) | Expr::Exp(a, b) => {
                    stack.push(StackItem { expr: item.expr, visited: true });
                    stack.push(StackItem { expr: b, visited: false });
                    stack.push(StackItem { expr: a, visited: false });
                }
                Expr::Metavar(s) => {
                    let ident = syn::Ident::new(s, proc_macro2::Span::call_site());
                    results.push(quote! { #ident });
                }
            }
        }
    }

    assert_eq!(results.len(), 1);

    results.pop().unwrap()
}
