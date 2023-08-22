use proc_macro::{Ident, TokenStream, TokenTree};

#[proc_macro_attribute]
pub fn nacos(attr: TokenStream, item: TokenStream) -> TokenStream {
    // TODO
    let retItem: TokenStream = item
        .into_iter()
        .filter(|item: &proc_macro::TokenTree| match item {
            Ident => {
                return true;
            }
            _ => {
                return true;
            }
        })
        .collect();
    retItem
}
