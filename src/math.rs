#[derive(Default, Debug, Clone, Copy)]
pub struct Vector3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3D {
    pub fn dot(&self, rhs: Vector3D) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(self) -> Self {
        self / self.length()
    }
}

impl std::ops::Add<Vector3D> for Vector3D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vector3D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Sub<Vector3D> for Vector3D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector3D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::Mul<Vector3D> for Vector3D {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vector3D {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl std::ops::Mul<f32> for Vector3D {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector3D {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl std::ops::Div<Vector3D> for Vector3D {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Vector3D {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl std::ops::Div<f32> for Vector3D {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Vector3D {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl std::ops::Neg for Vector3D {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vector3D {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
