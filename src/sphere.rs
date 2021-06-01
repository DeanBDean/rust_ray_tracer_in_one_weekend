use crate::hit::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::Vec3;
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sphere {
  center: Vec3,
  radius: f32,
}

impl Sphere {
  pub fn new(center: Vec3, radius: f32) -> Self {
    Self { center, radius }
  }
  pub fn center(&self) -> &Vec3 {
    &self.center
  }
  pub fn radius(&self) -> f32 {
    self.radius
  }
}

impl Hittable for Sphere {
  fn is_hit(&self, ray: &Ray, scalar_from_ray_origin_min: f32, scalar_from_ray_origin_max: f32) -> Option<HitRecord> {
    let center_offset = ray.origin() - self.center();
    let a = ray.direction().dot(ray.direction());
    let b = center_offset.dot(ray.direction());
    let c = center_offset.dot(&center_offset) - self.radius * self.radius;
    #[allow(clippy::style)]
    let discriminant = b * b - a * c;
    if discriminant > 0.0 {
      #[allow(clippy::style)]
      let scalar_lengths = [(-b - (b * b - a * c).sqrt()) / a, (-b + (b * b - a * c).sqrt()) / a];
      return scalar_lengths
        .iter()
        .fold_while(None::<HitRecord>, |_, scalar_length| {
          if *scalar_length < scalar_from_ray_origin_max && *scalar_length > scalar_from_ray_origin_min {
            let point_at_parameter = ray.point_at_parameter(*scalar_length);
            let normal = (point_at_parameter - self.center()) / self.radius;
            Done(Some(HitRecord::new(*scalar_length, &point_at_parameter, &normal)))
          } else {
            Continue(None)
          }
        })
        .into_inner();
    }
    None
  }
}
