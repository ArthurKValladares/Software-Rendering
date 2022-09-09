use crate::{Color, Vector3D};

pub struct Sphere {
    pub center: Vector3D,
    pub radius: f32,
    pub color: Color,
    pub specular: Option<f32>,
}
