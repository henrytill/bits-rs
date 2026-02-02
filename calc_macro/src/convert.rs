use proc_macro2::TokenStream;
use quote::quote;

use calc::syntax::{Expr, Op};

enum StackItem<'a> {
    Visit(&'a Expr),
    Emit(Op),
}

pub fn expr_to_syntax(expr: &Expr) -> TokenStream {
    let mut stack = vec![StackItem::Visit(expr)];
    let mut results = Vec::new();

    while let Some(item) = stack.pop() {
        match item {
            StackItem::Emit(op) => {
                let result = if matches!(op, Op::Neg) {
                    let a = results.pop().unwrap();
                    quote! {
                        calc::syntax::Expr::Neg(Box::new(#a))
                    }
                } else {
                    let b = results.pop().unwrap();
                    let a = results.pop().unwrap();
                    match op {
                        Op::Add => quote! {
                            calc::syntax::Expr::Add(Box::new(#a), Box::new(#b))
                        },
                        Op::Sub => quote! {
                            calc::syntax::Expr::Sub(Box::new(#a), Box::new(#b))
                        },
                        Op::Mul => quote! {
                            calc::syntax::Expr::Mul(Box::new(#a), Box::new(#b))
                        },
                        Op::Exp => quote! {
                            calc::syntax::Expr::Exp(Box::new(#a), Box::new(#b))
                        },
                        Op::Neg => unreachable!(),
                    }
                };
                results.push(result);
            }
            StackItem::Visit(expr) => match expr {
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
                Expr::Neg(a) => {
                    stack.push(StackItem::Emit(Op::Neg));
                    stack.push(StackItem::Visit(a));
                }
                Expr::Add(a, b) | Expr::Sub(a, b) | Expr::Mul(a, b) | Expr::Exp(a, b) => {
                    let Some(op) = expr.op() else { unreachable!() };
                    stack.push(StackItem::Emit(op));
                    stack.push(StackItem::Visit(b));
                    stack.push(StackItem::Visit(a));
                }
                Expr::Metavar(s) => {
                    let ident = syn::Ident::new(s, proc_macro2::Span::call_site());
                    results.push(quote! { #ident });
                }
            },
        }
    }

    {
        assert_eq!(results.len(), 1);
        results.pop().unwrap()
    }
}
