use std::ops;

#[derive(Debug)]
struct Vector3 {
    x: f64,
    y: f64,
    z: f64
}

impl Vector3 {
    fn norm(&self) -> Vector3 {
        self * (1.0 / self.length())
    }
    fn length(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }
}

impl ops::Add<&Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl ops::Sub<&Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl ops::Mul<f64> for &Vector3 {
    type Output = Vector3;

    fn mul(self, scalair: f64) -> Vector3 {
        Vector3 {
            x: self.x * scalair,
            y: self.y * scalair,
            z: self.z * scalair,
        }
    }
}

fn main() {
    let v1 = Vector3 { x: 4.0, y: 3.0, z: 0.0 };
    let v2 = Vector3 { x: 3.0, y: 2.0, z: 0.0 };
    println!("{:?}", v1.norm());
}
