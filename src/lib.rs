mod auto_from;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput,};

#[proc_macro_derive(Convert, attributes(convert, convert_field))]
pub fn attr_into2(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let context = auto_from::DeriveIntoContext::from(input);
    let out: TokenStream = context.render().into();
    out
}

