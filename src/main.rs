#![deny(clippy::perf, clippy::correctness, clippy::complexity, clippy::style, missing_debug_implementations)]
#![warn(clippy::pedantic)]

mod vec3;

use std::usize;
use vec3::Vec3;

#[allow(
  clippy::similar_names,
  clippy::cast_possible_truncation,
  clippy::cast_precision_loss,
  clippy::cast_sign_loss
)]
fn main() {
  let number_of_x_pixels = 200;
  let number_of_y_pixels = 100;
  println!("P3\n{} {}\n255", number_of_x_pixels, number_of_y_pixels);
  (0..number_of_y_pixels).rev().for_each(|current_y_pixel| {
    (0..number_of_x_pixels).for_each(|current_x_pixel| {
      let pixel_color = Vec3::new(
        current_x_pixel as f32 / number_of_x_pixels as f32,
        current_y_pixel as f32 / number_of_y_pixels as f32,
        0.2,
      );
      let red_value = (255.99 * pixel_color.r()) as usize;
      let green_value = (255.99 * pixel_color.g()) as usize;
      let blue_value = (255.99 * pixel_color.b()) as usize;
      println!("{} {} {}", red_value, green_value, blue_value);
    })
  })
}
