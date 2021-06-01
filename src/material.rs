use crate::hit::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;
use dyn_clone::{clone_trait_object, DynClone};
use std::fmt::Debug;

fn random_in_unit_sphere() -> Vec3 {
  let point_in_unit_sphere;
  loop {
    let potential_point_in_unit_sphere = 2.0 * Vec3::new(fastrand::f32(), fastrand::f32(), fastrand::f32()) - Vec3::new(1.0, 1.0, 1.0);
    if potential_point_in_unit_sphere.squared_length() < 1.0 {
      point_in_unit_sphere = potential_point_in_unit_sphere;
      break;
    }
  }
  point_in_unit_sphere
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScatterResult {
  attenuation: Vec3,
  scattered: Ray,
}

impl ScatterResult {
  pub fn new(attenuation: Vec3, scattered: Ray) -> Self {
    Self { attenuation, scattered }
  }
}

impl ScatterResult {
  pub fn attenuation(&self) -> &Vec3 {
    &self.attenuation
  }
  pub fn scattered(&self) -> &Ray {
    &self.scattered
  }
}

pub trait Material: Debug + DynClone {
  fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterResult>;
}

clone_trait_object!(Material);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Lambertian {
  albedo: Vec3,
}

impl Lambertian {
  pub fn new(albedo: &Vec3) -> Self {
    Self { albedo: *albedo }
  }
  pub fn albedo(&self) -> &Vec3 {
    &self.albedo
  }
}

impl Material for Lambertian {
  fn scatter(&self, _: &Ray, hit_record: &HitRecord) -> Option<ScatterResult> {
    let target = hit_record.point() + hit_record.normal() + random_in_unit_sphere();
    let scattered = Ray::new(hit_record.point(), &(target - hit_record.point()));
    Some(ScatterResult::new(self.albedo, scattered))
  }
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
  v - 2.0 * v.dot(n) * n
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Metal {
  albedo: Vec3,
  fuzz: f32,
}

impl Metal {
  pub fn new(albedo: &Vec3, fuzz: f32) -> Self {
    let fuzz = if fuzz < 0.0 { 0.0 } else { fuzz };
    Self { albedo: *albedo, fuzz }
  }
  pub fn albedo(&self) -> &Vec3 {
    &self.albedo
  }
  pub fn fuzz(&self) -> f32 {
    self.fuzz
  }
}

impl Material for Metal {
  fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterResult> {
    let reflected = reflect(&ray_in.direction().unit_vector(), hit_record.normal());
    let scattered = Ray::new(hit_record.point(), &(reflected + self.fuzz() * random_in_unit_sphere()));
    if scattered.direction().dot(hit_record.normal()) > 0.0 {
      Some(ScatterResult::new(self.albedo, scattered))
    } else {
      None
    }
  }
}
