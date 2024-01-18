use proc_macro::TokenStream;

#[proc_macro_derive(ComponentDerive)]
pub fn component_derive(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}
