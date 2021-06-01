#![deny(
  clippy::await_holding_lock,
  clippy::dbg_macro,
  clippy::debug_assert_with_mut_call,
  clippy::doc_markdown,
  clippy::empty_enum,
  clippy::enum_glob_use,
  clippy::exit,
  clippy::explicit_into_iter_loop,
  clippy::filter_map_next,
  clippy::fn_params_excessive_bools,
  clippy::if_let_mutex,
  clippy::imprecise_flops,
  clippy::inefficient_to_string,
  clippy::large_types_passed_by_value,
  clippy::let_unit_value,
  clippy::linkedlist,
  clippy::lossy_float_literal,
  clippy::macro_use_imports,
  clippy::map_err_ignore,
  clippy::map_flatten,
  clippy::map_unwrap_or,
  clippy::match_on_vec_items,
  clippy::match_same_arms,
  clippy::match_wildcard_for_single_variants,
  clippy::mem_forget,
  clippy::mismatched_target_os,
  clippy::needless_borrow,
  clippy::needless_continue,
  clippy::option_option,
  clippy::pub_enum_variant_names,
  clippy::ref_option_ref,
  clippy::rest_pat_in_fully_bound_structs,
  clippy::string_add_assign,
  clippy::string_add,
  clippy::string_to_string,
  clippy::suboptimal_flops,
  clippy::todo,
  clippy::unimplemented,
  clippy::unnested_or_patterns,
  clippy::unused_self,
  clippy::verbose_file_reads,
  clippy::correctness,
  clippy::complexity,
  clippy::perf,
  clippy::style,
  missing_debug_implementations,
  future_incompatible,
  nonstandard_style,
  rust_2018_idioms
)]
#![warn(clippy::pedantic)]

#[allow(unused_extern_crates)]
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
