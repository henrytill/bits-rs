mod convert;

use proc_macro::TokenStream;
use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn calc(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    match calc::parser::parse_expr(&input.value()) {
        Ok(expr) => convert::expr_to_syntax(&expr).into(),
        Err(e) => syn::Error::new(input.span(), format!("Parse error: {}", e))
            .to_compile_error()
            .into(),
    }
}
