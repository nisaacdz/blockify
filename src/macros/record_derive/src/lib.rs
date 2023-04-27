///! # Record - **record_derive**
///! 
///! This is a derive macro for the `Record` trait of `blockify` crate.
///! This does not override any of the functions within the `Record` trait.
///! 
///! # Usage 
///! ```
///! use blockify::trans::record::Record;
///! use serde::{Serialize, Deserialize}; 
///! 
///! #[derive(Debug, Clone, Serialize, Deserialize, Record)] 
///! pub struct MarriageContract {
///!   bride: String,
///!   groom: String, 
///! }
///!
///! #[derive(Serialize, Deserialize, Record)] 
///! pub struct Detail<T> {
///!   val: T 
///! }
///! ```


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
    let generics = &input.generics;
    let (impl_generics, ty_generics, _) = generics.split_for_impl();

    let gen = quote! {
        impl #impl_generics Record for #name #ty_generics {
            // ...
        }
    };

    gen.into()
}
