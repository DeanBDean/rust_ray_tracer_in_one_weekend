#![deny(clippy::perf, clippy::correctness, clippy::complexity, clippy::style, missing_debug_implementations)]
#![warn(clippy::pedantic)]

use std::{
  convert::TryFrom,
  fmt::{Display, Formatter, Result as FmtResult},
  ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Neg, Sub, SubAssign},
};
use try_from_integers::TryFromIntegers;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3([f32; 3]);

impl Vec3 {
  pub fn new_empty() -> Self {
    Self([0.0, 0.0, 0.0])
  }
  pub fn new(x: f32, y: f32, z: f32) -> Self {
    Self([x, y, z])
  }
  pub fn new_from_array(array: [f32; 3]) -> Self {
    Self(array)
  }
  /// Creates a new Vec3 from an array of 3 strings representing x, y and z
  ///
  /// # Errors
  ///
  /// Returns an error if any of the strings cannot be parsed into floats
  pub fn new_from_str_array<S: AsRef<str>>(input_strings: &[S; 3]) -> Result<Self, Error> {
    let inner = |input_strings: [&str; 3]| {
      let x: f32 = match input_strings[0].parse() {
        Ok(x) => x,
        Err(err) => {
          return Err(Box::new(err).into());
        }
      };
      let y: f32 = match input_strings[1].parse() {
        Ok(y) => y,
        Err(err) => {
          return Err(Box::new(err).into());
        }
      };
      let z: f32 = match input_strings[2].parse() {
        Ok(z) => z,
        Err(err) => {
          return Err(Box::new(err).into());
        }
      };
      Ok(Self::new(x, y, z))
    };
    inner([input_strings[0].as_ref(), input_strings[1].as_ref(), input_strings[2].as_ref()])
  }
  pub fn x(&self) -> f32 {
    self.0[0]
  }
  pub fn y(&self) -> f32 {
    self.0[1]
  }
  pub fn z(&self) -> f32 {
    self.0[2]
  }
  pub fn r(&self) -> f32 {
    self.x()
  }
  pub fn g(&self) -> f32 {
    self.y()
  }
  pub fn b(&self) -> f32 {
    self.z()
  }
  pub fn unit_vector(&self) -> Self {
    self / self.length()
  }
  #[allow(clippy::must_use_candidate)]
  pub fn mutate_to_unit_vector(&mut self) -> &Self {
    *self = self.unit_vector();
    self
  }
  pub fn dot(&self, other: &Self) -> f32 {
    self.z().mul_add(other.z(), self.x().mul_add(other.x(), self.y() * other.y()))
  }
  pub fn cross(&self, other: &Self) -> Self {
    Self::new(
      self.y() * other.z() - self.z() * other.y(),
      -(self.x() * other.z() - self.z() * other.x()),
      self.x() * other.y() - self.y() * other.x(),
    )
  }
  pub fn squared_length(&self) -> f32 {
    self.z().mul_add(self.z(), self.x().mul_add(self.x(), self.y() * self.y()))
  }
  pub fn length(&self) -> f32 {
    self.squared_length().sqrt()
  }
}

impl Display for Vec3 {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(f, "{} {} {}", self.x(), self.y(), self.z())
  }
}

impl Neg for Vec3 {
  type Output = Self;

  fn neg(self) -> Self::Output {
    Self([-self.x(), -self.y(), -self.z()])
  }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Copy, Debug, PartialEq, TryFromIntegers)]
enum Vec3Index {
  Zero,
  One,
  Two,
}

impl Index<Vec3Index> for Vec3 {
  type Output = f32;

  fn index(&self, vec3_index: Vec3Index) -> &Self::Output {
    match vec3_index {
      Vec3Index::Zero => &self.0[0],
      Vec3Index::One => &self.0[1],
      Vec3Index::Two => &self.0[2],
    }
  }
}

impl Add<Vec3> for Vec3 {
  type Output = Self;

  fn add(self, other: Self) -> Self::Output {
    Self([self.x() + other.x(), self.y() + other.y(), self.z() + other.z()])
  }
}

impl Add<Vec3> for &Vec3 {
  type Output = Vec3;

  fn add(self, other: Vec3) -> Self::Output {
    *self + other
  }
}

impl Add<&Vec3> for Vec3 {
  type Output = Vec3;

  fn add(self, other: &Self) -> Self::Output {
    self + *other
  }
}

impl<'first_vec, 'second_vec> Add<&'first_vec Vec3> for &'second_vec Vec3 {
  type Output = Vec3;

  fn add(self, other: &'first_vec Vec3) -> Self::Output {
    *self + *other
  }
}

impl AddAssign<Vec3> for Vec3 {
  fn add_assign(&mut self, other: Self) {
    *self = *self + other;
  }
}

impl Sub<Vec3> for Vec3 {
  type Output = Self;

  fn sub(self, other: Self) -> Self::Output {
    Self([self.x() - other.x(), self.y() - other.y(), self.z() - other.z()])
  }
}

impl Sub<Vec3> for &Vec3 {
  type Output = Vec3;

  fn sub(self, other: Vec3) -> Self::Output {
    *self - other
  }
}

impl Sub<&Vec3> for Vec3 {
  type Output = Vec3;

  fn sub(self, other: &Self) -> Self::Output {
    self - *other
  }
}

impl<'first_vec, 'second_vec> Sub<&'first_vec Vec3> for &'second_vec Vec3 {
  type Output = Vec3;

  fn sub(self, other: &'first_vec Vec3) -> Self::Output {
    *self - *other
  }
}

impl SubAssign<Vec3> for Vec3 {
  fn sub_assign(&mut self, other: Self) {
    *self = *self - other;
  }
}

impl Mul<f32> for Vec3 {
  type Output = Self;

  fn mul(self, other: f32) -> Self::Output {
    Self([self.x() * other, self.y() * other, self.z() * other])
  }
}

impl Mul<f32> for &Vec3 {
  type Output = Vec3;

  fn mul(self, other: f32) -> Self::Output {
    *self * other
  }
}

impl Mul<&Vec3> for f32 {
  type Output = Vec3;

  fn mul(self, other: &Vec3) -> Self::Output {
    self * *other
  }
}

impl MulAssign<f32> for Vec3 {
  fn mul_assign(&mut self, other: f32) {
    *self = *self * other;
  }
}

impl Mul<Vec3> for f32 {
  type Output = Vec3;

  fn mul(self, other: Vec3) -> Vec3 {
    other * self
  }
}

impl Mul<Vec3> for Vec3 {
  type Output = Self;

  fn mul(self, other: Self) -> Self::Output {
    Self([self.x() * other.x(), self.y() * other.y(), self.z() * other.z()])
  }
}

impl Mul<Vec3> for &Vec3 {
  type Output = Vec3;

  fn mul(self, other: Vec3) -> Self::Output {
    *self * other
  }
}

impl Mul<&Vec3> for Vec3 {
  type Output = Vec3;

  fn mul(self, other: &Vec3) -> Self::Output {
    self * *other
  }
}

impl<'first_vec, 'second_vec> Mul<&'first_vec Vec3> for &'second_vec Vec3 {
  type Output = Vec3;

  fn mul(self, other: &'first_vec Vec3) -> Self::Output {
    *self * *other
  }
}

impl MulAssign<Vec3> for Vec3 {
  fn mul_assign(&mut self, other: Self) {
    *self = *self * other;
  }
}

impl Div<f32> for Vec3 {
  type Output = Self;

  fn div(self, other: f32) -> Self::Output {
    Self([self.x() / other, self.y() / other, self.z() / other])
  }
}

impl Div<f32> for &Vec3 {
  type Output = Vec3;

  fn div(self, other: f32) -> Self::Output {
    *self / other
  }
}

impl Div<Vec3> for f32 {
  type Output = Vec3;

  fn div(self, other: Vec3) -> Self::Output {
    other * 1.0 / self
  }
}

impl Div<&Vec3> for f32 {
  type Output = Vec3;

  fn div(self, other: &Vec3) -> Self::Output {
    self / *other
  }
}

impl DivAssign<f32> for Vec3 {
  fn div_assign(&mut self, other: f32) {
    *self = *self / other;
  }
}

impl Div<Vec3> for Vec3 {
  type Output = Self;

  fn div(self, other: Self) -> Self::Output {
    Self([self.x() / other.x(), self.y() / other.y(), self.z() / other.z()])
  }
}

impl Div<Vec3> for &Vec3 {
  type Output = Vec3;

  fn div(self, other: Vec3) -> Self::Output {
    *self / other
  }
}

impl Div<&Vec3> for Vec3 {
  type Output = Vec3;

  fn div(self, other: &Vec3) -> Self::Output {
    self / *other
  }
}

impl<'first_vec, 'second_vec> Div<&'first_vec Vec3> for &'second_vec Vec3 {
  type Output = Vec3;

  fn div(self, other: &'first_vec Vec3) -> Self::Output {
    *self / *other
  }
}

impl DivAssign<Vec3> for Vec3 {
  fn div_assign(&mut self, other: Self) {
    *self = *self / other;
  }
}
