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

mod ray;
mod vec3;

use ray::Ray;
use std::usize;
use vec3::Vec3;

fn is_sphere_hit(center: &Vec3, radius: f32, ray: &Ray) -> bool {
  let center_offset = ray.origin() - center;
  let a = ray.direction().dot(ray.direction());
  let b = 2.0 * center_offset.dot(ray.direction());
  let c = center_offset.dot(&center_offset) - radius * radius;
  let discriminant = b * b - 4.0 * a * c;
  discriminant > 0.0
}

fn color(ray: &Ray) -> Vec3 {
  if is_sphere_hit(&Vec3::new(0.0, 0.0, -1.0), 0.5, ray) {
    return Vec3::new(1.0, 0.0, 0.0);
  }
  let unit_direction = ray.direction().unit_vector();
  let lerp_factor = 0.5 * (unit_direction.y() + 1.0);
  (1.0 - lerp_factor) as f32 * Vec3::new(1.0, 1.0, 1.0) + lerp_factor as f32 * Vec3::new(0.5, 0.7, 1.0)
}

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
  let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
  let horizontal = Vec3::new(4.0, 0.0, 0.0);
  let vertical = Vec3::new(0.0, 2.0, 0.0);
  let origin = Vec3::new(0.0, 0.0, 0.0);
  (0..number_of_y_pixels).rev().for_each(|current_y_pixel| {
    (0..number_of_x_pixels).for_each(|current_x_pixel| {
      let u = current_x_pixel as f32 / number_of_x_pixels as f32;
      let v = current_y_pixel as f32 / number_of_y_pixels as f32;
      let ray = Ray::new(&origin, &(lower_left_corner + u * horizontal + v * vertical));
      let pixel_color = color(&ray);
      let red_value = (255.99 * pixel_color.r()) as usize;
      let green_value = (255.99 * pixel_color.g()) as usize;
      let blue_value = (255.99 * pixel_color.b()) as usize;
      println!("{} {} {}", red_value, green_value, blue_value);
    })
  })
}
