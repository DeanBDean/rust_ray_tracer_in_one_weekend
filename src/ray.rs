#![deny(clippy::perf, clippy::correctness, clippy::complexity, clippy::style, missing_debug_implementations)]
#![warn(clippy::pedantic)]

use crate::vec3::Vec3;
use std::borrow::Cow;

#[derive(Clone, Debug, PartialEq)]
pub struct Ray<'a, 'b> {
  origin: Cow<'a, Vec3>,
  direction: Cow<'b, Vec3>,
}

#[allow(dead_code)]
impl<'a, 'b> Ray<'a, 'b> {
  pub fn new(origin: Cow<'a, Vec3>, direction: Cow<'b, Vec3>) -> Self {
    Self { origin, direction }
  }
  pub fn origin(&self) -> &Cow<'a, Vec3> {
    &self.origin
  }
  pub fn direction(&self) -> &Cow<'b, Vec3> {
    &self.direction
  }
  pub fn point_at_parameter(&self, scalar_length: f32) -> Vec3 {
    *self.origin + scalar_length * *self.direction
  }
}
