#![deny(clippy::perf, clippy::correctness, clippy::complexity, clippy::style, missing_debug_implementations)]
#![warn(clippy::pedantic)]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(TryFromIntegers)]
pub fn derive_try_from_impls(enum_or_struct: TokenStream) -> TokenStream {
  let DeriveInput { ident, .. } = parse_macro_input!(enum_or_struct);
  let integer_types = [
    quote!(u8),
    quote!(u16),
    quote!(u32),
    quote!(u64),
    quote!(u128),
    quote!(usize),
    quote!(i8),
    quote!(i16),
    quote!(i32),
    quote!(i64),
    quote!(i128),
    quote!(isize),
  ];

  integer_types
    .iter()
    .fold(TokenStream::new(), |mut total_tokenstream, current_integer_type| {
      let new_try_from = quote!(
        impl TryFrom<#current_integer_type> for #ident {
          type Error = Error;

          fn try_from(value: #current_integer_type) -> Result<Self, Self::Error> {
            match value {
              0 => Ok(Self::Zero),
              1 => Ok(Self::One),
              2 => Ok(Self::Two),
              _ => Err("Vec3Index must be 0, 1 or 2".into()),
            }
          }
        }
      );
      total_tokenstream.extend(TokenStream::from(new_try_from));
      total_tokenstream
    })
}
