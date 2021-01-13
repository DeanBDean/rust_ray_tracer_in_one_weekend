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
  clippy::cargo,
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

use std::usize;

#[allow(
  clippy::cast_possible_truncation,
  clippy::cast_precision_loss,
  clippy::cast_sign_loss,
  clippy::similar_names
)]
fn main() {
  let number_of_x_pixels = 200;
  let number_of_y_pixels = 100;
  println!("P3\n{} {}\n255", number_of_x_pixels, number_of_y_pixels);
  (0..number_of_y_pixels).rev().for_each(|current_y_pixel| {
    (0..number_of_x_pixels).for_each(|current_x_pixel| {
      let red_value = current_x_pixel as f32 / number_of_x_pixels as f32;
      let green_value = current_y_pixel as f32 / number_of_y_pixels as f32;
      let blue_value = 0.2;
      let red_value = (255.99 * red_value) as usize;
      let green_value = (255.99 * green_value) as usize;
      let blue_value = (255.99 * blue_value) as usize;
      println!("{} {} {}", red_value, green_value, blue_value);
    })
  })
}
