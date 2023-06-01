//! # Record - **record_derive**
//!
//! A derive macro for the `blockify::record::Record` blockify.
//!
//! Types deriving `Record` must implement `Serialize + Deserialize`
//!
//!
//! # Usage
//! ```
//! use blockify::record::Record;
//! use serde::{Serialize, Deserialize};
//!
//! #[derive(Serialize, Deserialize, Record)]
//! pub struct MarriageContract {
//!   bride: String,
//!   groom: String,
//! }
//!
//! #[derive(Serialize, Deserialize, Record)]
//! pub struct Detail<T> {
//!   val: T
//! }
//! ```

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
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let gen = quote! {
        impl #impl_generics Record for #name #ty_generics #where_clause {
            fn sign(
                &self,
                key: &blockify::AuthKeyPair,
            ) -> Result<blockify::DigitalSignature, blockify::SigningError> {
                let msg = blockify::serialize(self).map_err(|e| SigningError::SerdeError(e))?;
                let signature = blockify::sign_msg(&msg, key)?;
                Ok(signature)
            }

            fn verify(
                &self,
                signature: &blockify::DigitalSignature,
                key: &blockify::PublicKey,
            ) -> Result<(), blockify::VerificationError> {
                let msg =
                    blockify::serialize(self).map_err(|e| crate::VerificationError::SerdeError(e))?;
                key.verify(&msg, signature)
            }

            fn record(
                self,
                keypair: blockify::AuthKeyPair,
                metadata: blockify::data::Metadata,
            ) -> Result<blockify::record::SignedRecord<Self>, blockify::SigningError> {
                let signature = self.sign(&keypair)?;
                let hash = self.hash();
                Ok(blockify::record::SignedRecord::new(
                    self,
                    signature,
                    keypair.into_public_key(),
                    hash,
                    metadata,
                ))
            }

            fn hash(&self) -> blockify::Hash {
                blockify::hash(self)
            }
        }
    };

    gen.into()
}
