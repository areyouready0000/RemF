use std::ops::Add;

struct Vector2 {
    coords: [f64; 2]
}

impl Vector2 {
    pub fn new(coords: &[f64; 2]) -> Self {
        Self { coords: *coords }
    }

    pub fn get(&self) -> &[f64; 2] {
        &self.coords
    }

    pub fn x(&self) -> f64 {
        self.coords[0]
    }
    pub fn y(&self) -> f64 {
        self.coords[1]
    }

    pub fn cross(&self, other: &Self) -> f64 {
        self.x() * other.y() - other.x() * self.y()
    }
}

impl Add for Vector2 {
    type Output = Vector2;

    fn add(self, other: Vector2) -> Self::Output {
        Self::Output {coords: [
            self.coords[0] + other.coords[0],
            self.coords[1] + other.coords[1]
        ]}
    }
}

impl Add<f64> for Vector2 {
    type Output = Vector2;

    fn add(self, other: f64) -> Self::Output {
        Self::Output {coords: [
            self.coords[0] + other,
            self.coords[1] + other
        ]}
    }
}