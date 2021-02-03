#![deny(clippy::perf, clippy::correctness, clippy::complexity, clippy::style, missing_debug_implementations)]
#![warn(clippy::pedantic)]

use crate::ray::Ray;
use crate::vec3::Vec3;
use std::f32::consts::PI;

fn random_in_unit_disk() -> Vec3 {
  let point_in_unit_disk;
  loop {
    let potential_point_in_unit_disk = 2.0 * Vec3::new(fastrand::f32(), fastrand::f32(), fastrand::f32()) - Vec3::new(1.0, 1.0, 0.0);
    if potential_point_in_unit_disk.squared_length() < 1.0 {
      point_in_unit_disk = potential_point_in_unit_disk;
      break;
    }
  }
  point_in_unit_disk
}

#[allow(dead_code)]
pub struct Camera {
  origin: Vec3,
  lower_left_corner: Vec3,
  horizontal: Vec3,
  vertical: Vec3,
  u: Vec3,
  v: Vec3,
  w: Vec3,
  lens_radius: f32,
}

#[allow(dead_code)]
impl Camera {
  #[allow(clippy::too_many_arguments)]
  pub fn new(
    origin: &Vec3,
    horizontal: &Vec3,
    vertical: &Vec3,
    u: &Vec3,
    v: &Vec3,
    w: &Vec3,
    lower_left_corner: &Vec3,
    lens_radius: f32,
  ) -> Self {
    Self {
      origin: *origin,
      horizontal: *horizontal,
      vertical: *vertical,
      lower_left_corner: *lower_left_corner,
      lens_radius,
      u: *u,
      v: *v,
      w: *w,
    }
  }
  pub fn new_from_fov_and_aspect(
    look_from: &Vec3,
    look_at: &Vec3,
    vup: &Vec3,
    vertical_fov: f32,
    aspect: f32,
    aperture: f32,
    focus_distance: f32,
  ) -> Self {
    let lens_radius = aperture / 2.0;
    let theta = vertical_fov * (PI / 180.0);
    let half_height = (theta / 2.0).tan();
    let half_width = aspect * half_height;
    let origin = look_from;
    let w = (look_from - look_at).unit_vector();
    let u = vup.cross(&w).unit_vector();
    let v = w.cross(&u);
    let lower_left_corner = origin - half_width * focus_distance * u - half_height * focus_distance * v - focus_distance * w;
    let horizontal = 2.0 * half_width * focus_distance * u;
    let vertical = 2.0 * half_height * focus_distance * v;
    Self {
      origin: *origin,
      horizontal,
      vertical,
      lower_left_corner,
      u,
      v,
      w,
      lens_radius,
    }
  }
  pub fn get_ray(&self, s: f32, t: f32) -> Ray {
    let random_position_on_lens_disk = self.lens_radius * random_in_unit_disk();
    let offset = self.u * random_position_on_lens_disk.x() + self.v * random_position_on_lens_disk.y();
    Ray::new(
      (self.origin + offset).into(),
      (self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset).into(),
    )
  }
}
