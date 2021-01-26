#![deny(clippy::perf, clippy::correctness, clippy::complexity, clippy::style, missing_debug_implementations)]
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
          return **scatter_result.attenuation() * color(&scatter_result.scattered(), world, depth + 1);
        }
      }
      Vec3::new(0.0, 0.0, 0.0)
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
  world.list_mut().push(Box::new(Sphere::new(
    Vec3::new(0.0, 0.0, -1.0).into(),
    0.5,
    Arc::new(Lambertian::new(Vec3::new(0.8, 0.3, 0.3).into())),
  )));
  world.list_mut().push(Box::new(Sphere::new(
    Vec3::new(0.0, -100.5, -1.0).into(),
    100.0,
    Arc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0).into())),
  )));
  world.list_mut().push(Box::new(Sphere::new(
    Vec3::new(1.0, 0.0, -1.0).into(),
    0.5,
    Arc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2).into(), 0.3)),
  )));
  world.list_mut().push(Box::new(Sphere::new(
    Vec3::new(-1.0, 0.0, -1.0).into(),
    0.5,
    Arc::new(Metal::new(Vec3::new(0.8, 0.8, 0.8).into(), 1.0)),
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
