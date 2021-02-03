#![deny(clippy::perf, clippy::correctness, clippy::complexity, clippy::style, missing_debug_implementations)]
#![warn(clippy::pedantic)]

use crate::ray::Ray;
use crate::vec3::Vec3;
use std::f32::consts::PI;

pub struct Camera {
  origin: Vec3,
  lower_left_corner: Vec3,
  horizontal: Vec3,
  vertical: Vec3,
}

#[allow(dead_code)]
impl Camera {
  pub fn new(origin: &Vec3, horizontal: &Vec3, vertical: &Vec3, lower_left_corner: &Vec3) -> Self {
    Self {
      origin: *origin,
      horizontal: *horizontal,
      vertical: *vertical,
      lower_left_corner: *lower_left_corner,
    }
  }
  pub fn new_from_fov_and_aspect(look_from: &Vec3, look_at: &Vec3, vup: &Vec3, vertical_fov: f32, aspect: f32) -> Self {
    let theta = vertical_fov * (PI / 180.0);
    let half_height = (theta / 2.0).tan();
    let half_width = aspect * half_height;
    let origin = look_from;
    let w = (look_from - look_at).unit_vector();
    let u = vup.cross(&w).unit_vector();
    let v = w.cross(&u);
    let lower_left_corner = origin - half_width * u - half_height * v - w;
    let horizontal = 2.0 * half_width * u;
    let vertical = 2.0 * half_height * v;
    Self {
      origin: *origin,
      horizontal,
      vertical,
      lower_left_corner,
    }
  }
  pub fn default() -> Self {
    Self {
      origin: Vec3::new(0.0, 0.0, 0.0),
      lower_left_corner: Vec3::new(-2.0, -1.0, -1.0),
      horizontal: Vec3::new(4.0, 0.0, 0.0),
      vertical: Vec3::new(0.0, 2.0, 0.0),
    }
  }
  pub fn get_ray(&self, s: f32, t: f32) -> Ray {
    Ray::new(
      (&self.origin).into(),
      (self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin).into(),
    )
  }
}
