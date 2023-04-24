// record_derive/src/lib.rs

extern crate proc_macro;

use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Record)]
pub fn record_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input code as a Rust syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    impl_record(&input)
}

fn impl_record(input: &DeriveInput) -> proc_macro::TokenStream {
    let name = &input.ident;

    let gen = quote! {
        impl Record for #name {}
    };

    gen.into()
}