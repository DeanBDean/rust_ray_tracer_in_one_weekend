#![deny(clippy::perf, clippy::correctness, clippy::complexity, clippy::style, missing_debug_implementations)]
#![warn(clippy::pedantic)]

use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::borrow::Cow;
use std::sync::Arc;

#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug)]
pub struct HitRecord<'a, 'b> {
  scalar_from_ray_origin: f32,
  point: Cow<'a, Vec3>,
  normal: Cow<'b, Vec3>,
  material: Arc<dyn Material>,
}

#[allow(dead_code)]
impl<'a, 'b> HitRecord<'a, 'b> {
  pub fn new(scalar_from_ray_origin: f32, point: Cow<'a, Vec3>, normal: Cow<'b, Vec3>, material: Arc<dyn Material>) -> Self {
    Self {
      scalar_from_ray_origin,
      point,
      normal,
      material,
    }
  }
  pub fn scalar_from_ray_origin(&self) -> f32 {
    self.scalar_from_ray_origin
  }
  pub fn point(&self) -> &Cow<'a, Vec3> {
    &self.point
  }
  pub fn normal(&self) -> &Cow<'b, Vec3> {
    &self.normal
  }
  pub fn material(&self) -> Arc<dyn Material> {
    self.material.clone()
  }
}

pub trait Hittable {
  fn is_hit(&self, ray: &Ray, scalar_from_ray_origin_min: f32, scalar_from_ray_origin_max: f32) -> Option<HitRecord>;
}

pub struct HittableList(Vec<Box<dyn Hittable>>);

impl HittableList {
  pub fn new() -> Self {
    Self(Vec::<Box<dyn Hittable>>::new())
  }
  pub fn list(&self) -> &Vec<Box<dyn Hittable>> {
    &self.0
  }
  pub fn list_mut(&mut self) -> &mut Vec<Box<dyn Hittable>> {
    &mut self.0
  }
}

impl Hittable for HittableList {
  fn is_hit(&self, ray: &Ray, scalar_from_ray_origin_min: f32, scalar_from_ray_origin_max: f32) -> Option<HitRecord> {
    let mut closet_so_far = scalar_from_ray_origin_max;
    self.list().iter().fold(None::<HitRecord>, |accumulator, current_hittable| {
      current_hittable
        .is_hit(ray, scalar_from_ray_origin_min, closet_so_far)
        .map_or(accumulator, |hit_record| {
          closet_so_far = hit_record.scalar_from_ray_origin;
          Some(hit_record)
        })
    })
  }
}
