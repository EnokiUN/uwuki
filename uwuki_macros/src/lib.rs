use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn command(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("{:#?}\n{:#?}", attr, item);
    item
}
