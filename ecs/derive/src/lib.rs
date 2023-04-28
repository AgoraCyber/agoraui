use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemStruct};

mod component;
mod system;

#[proc_macro_attribute]
pub fn component(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemStruct);

    let args = parse_macro_input!(attr as component::Args);

    component::Component::new(args, item).generate()
}

#[proc_macro_attribute]
pub fn system_id(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemStruct);

    let args = parse_macro_input!(attr as system::Args);

    system::System::new(args, item).generate()
}
