use proc_macro2::TokenStream;
use quote::quote;

use calc::syntax::Expr;

pub fn expr_to_syntax(expr: &Expr) -> TokenStream {
    match expr {
        Expr::Var(x) => quote! {
            calc::syntax::Expr::Var(String::from(#x))
        },
        Expr::Const(n) => quote! {
            calc::syntax::Expr::Const(#n)
        },
        Expr::Neg(a) => {
            let a = expr_to_syntax(a);
            quote! {
                calc::syntax::Expr::Neg(Box::new(#a))
            }
        }
        Expr::Add(a, b) => {
            let a = expr_to_syntax(a);
            let b = expr_to_syntax(b);
            quote! {
                calc::syntax::Expr::Add(Box::new(#a), Box::new(#b))
            }
        }
        Expr::Sub(a, b) => {
            let a = expr_to_syntax(a);
            let b = expr_to_syntax(b);
            quote! {
                calc::syntax::Expr::Sub(Box::new(#a), Box::new(#b))
            }
        }
        Expr::Mul(a, b) => {
            let a = expr_to_syntax(a);
            let b = expr_to_syntax(b);
            quote! {
                calc::syntax::Expr::Mul(Box::new(#a), Box::new(#b))
            }
        }
        Expr::Exp(a, b) => {
            let a = expr_to_syntax(a);
            let b = expr_to_syntax(b);
            quote! {
                calc::syntax::Expr::Exp(Box::new(#a), Box::new(#b))
            }
        }
        Expr::Metavar(s) => {
            let ident = syn::Ident::new(s, proc_macro2::Span::call_site());
            quote! { #ident }
        }
    }
}
