use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};
use num_traits::Pow;

#[derive(Debug, Clone)]
struct Vector3 {
    x: f64,
    y: f64,
    z: f64
}

impl Vector3 {
    pub fn new(x: f64, y: f64,z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self::new(
            self.y * other.z - other.y * self.z,
            self.z * other.x - other.z * self.x,
            self.x * other.y - other.x * self.y
        )
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.x.pow(2) + self.y.pow(2) + self.z.pow(2))
    }

    pub fn length_squared(&self) -> f64 {
        self.x.pow(2) + self.y.pow(2) + self.z.pow(2)
    }

    pub fn normalize(&mut self) {
        let length: f64 = self.length();

        self.x /= length;
    }

    pub fn normalized(&self) -> Self {
        let length: f64 = self.length();

        Self::new(
            self.x / length,
            self.y / length,
            self.z / length,
        )
    }

    pub fn projected(&self, normal: &Self) -> Self {
        (*self).clone() - (*normal).clone() * self.dot(normal)
    }

    pub fn reflected(&self, normal: &Self) -> Self {
        (*self).clone() - (*normal).clone() * 2. * self.dot(normal)
    }

    pub fn tangential(&self) -> (Self, Self) {
        let mut a: Vector3;

        if self.y != 0. || self.z != 0. {
            a = Vector3::new(1., 0., 0.)
        } else {
            a = Vector3::new(0., 1., 0.).cross(self).normalized();
        }

        let b: Vector3 = self.cross(&a);

        (a, b)
    }
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
        )
    }
}

impl Add<f64> for Vector3 {
    type Output = Self;

    fn add(self, other: f64) -> Self::Output {
        Self::new(
            self.x + other,
            self.y + other,
            self.z + other,
        )
    }
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z
        )
    }
}

impl Sub<f64> for Vector3 {
    type Output = Self;

    fn sub(self, other: f64) -> Self::Output {
        Self::new(
            self.x - other,
            self.y - other,
            self.z - other
        )
    }
}

impl Mul for Vector3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self::new(
            self.x * other.x,
            self.y * other.y,
            self.z * other.z,
        )
    }
}

impl Mul<f64> for Vector3 {
    type Output = Self;

    fn mul(self, other: f64) -> Self::Output {
        Self::new(
            self.x * other,
            self.y * other,
            self.z * other,
        )
    }
}

impl Div for Vector3 {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self::new(
            self.x / other.x,
            self.y / other.y,
            self.z / other.z
        )
    }
}

impl Div<f64> for Vector3 {
    type Output = Self;

    fn div(self, other: f64) -> Self::Output {
        Self::new(
            self.x / other,
            self.y / other,
            self.z / other,
        )
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl AddAssign<f64> for Vector3 {
    fn add_assign(&mut self, other: f64) {
        self.x += other;
        self.y += other;
        self.z += other;
    }
}

impl SubAssign for Vector3 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl SubAssign<f64> for Vector3 {
    fn sub_assign(&mut self, other: f64) {
        self.x -= other;
        self.y -= other;
        self.z -= other;
    }
}

impl MulAssign for Vector3 {
    fn mul_assign(&mut self, other: Self) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}

impl MulAssign<f64> for Vector3 {
    fn mul_assign(&mut self, other: f64) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

impl DivAssign for Vector3 {
    fn div_assign(&mut self, other: Self) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
    }
}

impl DivAssign<f64> for Vector3 {
    fn div_assign(&mut self, other: f64) {
        self.x /= other;
        self.y /= other;
        self.z /= other;
    }
}

impl PartialEq for Vector3 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }

    fn ne(&self, other: &Self) -> bool {
        self.x != other.x || self.y != other.y || self.z != other.z
    }
}

#[cfg(test)]
mod vector3_tests {
    use super::*;

    #[test]
    fn test() {
        let mut vector = Vector3::new(1.0, 2.0, 3.0);
        let vector2 = Vector3::new(1.0, 0.0, 3.0);

        println!("{:?}", vector.normalized());
    }
}