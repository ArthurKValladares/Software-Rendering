#[derive(Default, Debug, Clone, Copy)]
pub struct Vector3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub fn dot(lhs: Vector3D, rhs: Vector3D) -> f32 {
    lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
}
