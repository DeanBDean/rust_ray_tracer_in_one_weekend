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

mod camera;
mod hit;
mod material;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;
use hit::{Hittable, HittableList};
use material::{Lambertian, Metal};
use ray::Ray;
use sphere::Sphere;
use std::sync::Arc;
use std::usize;
use vec3::Vec3;

fn color(ray: &Ray, world: &HittableList, depth: u8) -> Vec3 {
  world.is_hit(ray, 0.001, f32::MAX).map_or_else(
    || {
      let unit_direction = ray.direction().unit_vector();
      let lerp_factor = 0.5 * (unit_direction.y() + 1.0);
      (1.0 - lerp_factor) as f32 * Vec3::new(1.0, 1.0, 1.0) + lerp_factor as f32 * Vec3::new(0.5, 0.7, 1.0)
    },
    |hit_record| {
      if depth < 50 {
        if let Some(scatter_result) = hit_record.material().scatter(ray, &hit_record) {
          return scatter_result.attenuation() * color(scatter_result.scattered(), world, depth + 1);
        }
      }
      Vec3::new(0.0, 0.0, 0.0)
    },
  )
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
  let number_of_samples_per_pixel = 100;
  println!("P3\n{} {}\n255", number_of_x_pixels, number_of_y_pixels);
  let mut world = HittableList::new();
  world.list_mut().push(Box::new(Sphere::new(
    &Vec3::new(0.0, 0.0, -1.0),
    0.5,
    Arc::new(Lambertian::new(&Vec3::new(0.8, 0.3, 0.3))),
  )));
  world.list_mut().push(Box::new(Sphere::new(
    &Vec3::new(0.0, -100.5, -1.0),
    100.0,
    Arc::new(Lambertian::new(&Vec3::new(0.8, 0.8, 0.0))),
  )));
  world.list_mut().push(Box::new(Sphere::new(
    &Vec3::new(1.0, 0.0, -1.0),
    0.5,
    Arc::new(Metal::new(&Vec3::new(0.8, 0.6, 0.2), 0.3)),
  )));
  world.list_mut().push(Box::new(Sphere::new(
    &Vec3::new(-1.0, 0.0, -1.0),
    0.5,
    Arc::new(Metal::new(&Vec3::new(0.8, 0.8, 0.8), 1.0)),
  )));
  let camera = Camera::default();
  (0..number_of_y_pixels).rev().for_each(|current_y_pixel| {
    (0..number_of_x_pixels).for_each(|current_x_pixel| {
      let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
      (0..number_of_samples_per_pixel).for_each(|_| {
        let u = (current_x_pixel as f32 + fastrand::f32()) / number_of_x_pixels as f32;
        let v = (current_y_pixel as f32 + fastrand::f32()) / number_of_y_pixels as f32;
        let ray = camera.get_ray(u, v);
        pixel_color += color(&ray, &world, 0);
      });
      pixel_color /= number_of_samples_per_pixel as f32;
      pixel_color = Vec3::new(pixel_color.x().sqrt(), pixel_color.y().sqrt(), pixel_color.z().sqrt());
      let red_value = (255.99 * pixel_color.r()) as usize;
      let green_value = (255.99 * pixel_color.g()) as usize;
      let blue_value = (255.99 * pixel_color.b()) as usize;
      println!("{} {} {}", red_value, green_value, blue_value);
    })
  })
}
