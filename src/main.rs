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
use material::{Dielectric, Lambertian, Metal};
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

fn create_random_scene() -> HittableList {
  let mut random_array = HittableList::new();
  random_array.list_mut().push(
    Box::new(
      Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0).into(),
        1000.0,
        Arc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5).into())),
      )
    )
  );
  (-11..11).for_each(|a| {
    (-11..11).for_each(|b| {
      let choose_mat = fastrand::f32();
      #[allow(clippy::cast_precision_loss)]
      let center = Vec3::new(a as f32 + 0.9 * fastrand::f32(), 0.2, b as f32 + 0.9 * fastrand::f32());
      if (center - Vec3::new(4.0, 0.2, 2.0)).length() > 0.9 {
        if choose_mat < 0.8 {
          random_array.list_mut().push(
            Box::new(
              Sphere::new(
                center.into(),
                0.2,
                Arc::new(Lambertian::new(Vec3::new(fastrand::f32()*fastrand::f32(), fastrand::f32() * fastrand::f32(), fastrand::f32() * fastrand::f32()).into()))
              )
            )
          );
        } else if choose_mat < 0.95 {
          random_array.list_mut().push(
            Box::new(
              Sphere::new(
                center.into(),
                0.2,
                Arc::new(Metal::new(Vec3::new(0.5 * (1.0 + fastrand::f32()), 0.5 * (1.0 + fastrand::f32()), 0.5 * (1.0 + fastrand::f32())).into(), 0.5 * fastrand::f32()))
              )
            )
          )
        } else {
          random_array.list_mut().push(
            Box::new(
              Sphere::new(
                center.into(),
                0.2,
                Arc::new(Dielectric::new(1.5))
              )
            )
          )
        }
      }
    })
  });
  random_array.list_mut().push(
    Box::new(
      Sphere::new(
        Vec3::new(0.0, 1.0, 0.0).into(),
        1.0,
        Arc::new(Dielectric::new(1.5))
      )
    )
  );
  random_array.list_mut().push(
    Box::new(
      Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0).into(),
        1.0,
        Arc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1).into()))
      )
    )
  );
  random_array.list_mut().push(
    Box::new(
      Sphere::new(
        Vec3::new(4.0, 1.0, 0.0).into(),
        1.0,
        Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5).into(), 0.0))
      )
    )
  );
  random_array
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
  let world = create_random_scene();
  let look_from = Vec3::new(13.0, 2.0, 3.0);
  let look_at = Vec3::new(0.0, 0.0, 0.0);
  let distance_to_focus = 10.0;
  let aperature = 0.1;
  let camera = Camera::new_from_fov_and_aspect(
    &look_from,
    &look_at,
    &Vec3::new(0.0, 1.0, 0.0),
    20.0,
    number_of_x_pixels as f32 / number_of_y_pixels as f32,
    aperature,
    distance_to_focus,
  );
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
