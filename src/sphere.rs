#![deny(clippy::perf, clippy::correctness, clippy::complexity, clippy::style, missing_debug_implementations)]
#![warn(clippy::pedantic)]

use crate::hit::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use std::borrow::Cow;
use std::sync::Arc;

pub struct Sphere<'a> {
  center: Cow<'a, Vec3>,
  material: Arc<dyn Material>,
  radius: f32,
}

#[allow(dead_code)]
impl<'a> Sphere<'a> {
  pub fn new(center: Cow<'a, Vec3>, radius: f32, material: Arc<dyn Material>) -> Self {
    Self { center, radius, material }
  }
  pub fn center(&self) -> &Cow<'a, Vec3> {
    &self.center
  }
  pub fn radius(&self) -> f32 {
    self.radius
  }
  pub fn material(&self) -> Arc<dyn Material> {
    self.material.clone()
  }
}

impl<'a> Hittable for Sphere<'a> {
  fn is_hit(&self, ray: &Ray, scalar_from_ray_origin_min: f32, scalar_from_ray_origin_max: f32) -> Option<HitRecord> {
    let center_offset = *(*ray.origin()) - *(*self.center());
    let a = ray.direction().dot(ray.direction());
    let b = center_offset.dot(ray.direction());
    let c = center_offset.dot(&center_offset) - self.radius * self.radius;
    let discriminant = b * b - a * c;
    if discriminant > 0.0 {
      let scalar_lengths = [(-b - (b * b - a * c).sqrt()) / a, (-b + (b * b - a * c).sqrt()) / a];
      return scalar_lengths
        .iter()
        .fold_while(None::<HitRecord>, |_, scalar_length| {
          if *scalar_length < scalar_from_ray_origin_max && *scalar_length > scalar_from_ray_origin_min {
            let point_at_parameter = ray.point_at_parameter(*scalar_length);
            let normal = (point_at_parameter - *(*self.center())) / self.radius;
            Done(Some(HitRecord::new(
              *scalar_length,
              point_at_parameter.into(),
              normal.into(),
              self.material(),
            )))
          } else {
            Continue(None)
          }
        })
        .into_inner();
    }
    None
  }
}
