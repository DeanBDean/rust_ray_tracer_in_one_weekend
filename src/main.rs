#![deny(clippy::perf, clippy::correctness, clippy::complexity, clippy::style, missing_debug_implementations)]
#![warn(clippy::pedantic)]

mod camera;
mod hit;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;
use hit::{Hittable, HittableList};
use ray::Ray;
use sphere::Sphere;
use std::{borrow::Cow, usize};
use vec3::Vec3;

fn color(ray: &Ray, world: &HittableList) -> Vec3 {
  world.is_hit(ray, 0.0, f32::MAX).map_or_else(
    || {
      let unit_direction = ray.direction().unit_vector();
      let lerp_factor = 0.5 * (unit_direction.y() + 1.0);
      (1.0 - lerp_factor) as f32 * Vec3::new(1.0, 1.0, 1.0) + lerp_factor as f32 * Vec3::new(0.5, 0.7, 1.0)
    },
    |hit_record| {
      0.5
        * match *hit_record.normal() {
          Cow::Borrowed(normal) => Vec3::new(normal.x() + 1.0, normal.y() + 1.0, normal.z() + 1.0),
          Cow::Owned(normal) => Vec3::new(normal.x() + 1.0, normal.y() + 1.0, normal.z() + 1.0),
        }
    },
  )
}

#[allow(
  clippy::similar_names,
  clippy::cast_possible_truncation,
  clippy::cast_precision_loss,
  clippy::cast_sign_loss
)]
fn main() {
  let number_of_x_pixels = 200;
  let number_of_y_pixels = 100;
  let number_of_samples_per_pixel = 100;
  println!("P3\n{} {}\n255", number_of_x_pixels, number_of_y_pixels);
  let mut world = HittableList::new();
  world.list_mut().push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0).into(), 0.5)));
  world.list_mut().push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0).into(), 100.0)));
  let camera = Camera::default();
  (0..number_of_y_pixels).rev().for_each(|current_y_pixel| {
    (0..number_of_x_pixels).for_each(|current_x_pixel| {
      let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
      (0..number_of_samples_per_pixel).for_each(|_| {
        let u = (current_x_pixel as f32 + fastrand::f32()) / number_of_x_pixels as f32;
        let v = (current_y_pixel as f32 + fastrand::f32()) / number_of_y_pixels as f32;
        let ray = camera.get_ray(u, v);
        pixel_color += color(&ray, &world);
      });
      pixel_color /= number_of_samples_per_pixel as f32;
      let red_value = (255.99 * pixel_color.r()) as usize;
      let green_value = (255.99 * pixel_color.g()) as usize;
      let blue_value = (255.99 * pixel_color.b()) as usize;
      println!("{} {} {}", red_value, green_value, blue_value);
    })
  })
}
