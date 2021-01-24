#![deny(clippy::perf, clippy::correctness, clippy::complexity, clippy::style, missing_debug_implementations)]
#![warn(clippy::pedantic)]

use crate::ray::Ray;
use crate::vec3::Vec3;

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
  pub fn default() -> Self {
    Self {
      origin: Vec3::new(0.0, 0.0, 0.0),
      lower_left_corner: Vec3::new(-2.0, -1.0, -1.0),
      horizontal: Vec3::new(4.0, 0.0, 0.0),
      vertical: Vec3::new(0.0, 2.0, 0.0),
    }
  }
  pub fn get_ray(&self, u: f32, v: f32) -> Ray {
    Ray::new(
      (&self.origin).into(),
      (self.lower_left_corner + u * self.horizontal + v * self.vertical).into(),
    )
  }
}
