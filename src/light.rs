use crate::{color::Color, math::Vector3D};

pub struct AmbientData {
    pub color: Color,
}

pub struct PointData {
    pub color: Color,
    pub position: Vector3D,
}

pub struct DirectionalData {
    pub color: Color,
    pub direction: Vector3D,
}

pub enum Light {
    Ambient(AmbientData),
    Point(PointData),
    Directional(DirectionalData),
}

impl Light {
    pub fn compute_color(&self, point: Vector3D, normal: Vector3D) -> Color {
        match self {
            Light::Ambient(data) => data.color,
            Light::Point(data) => {
                let direction = data.position - point;
                let denominator = point.dot(direction);
                if denominator > 0.0 {
                    data.color * denominator / (point.length() * normal.length())
                } else {
                    Color::black()
                }
            }
            Light::Directional(data) => {
                let direction = data.direction;
                let denominator = point.dot(direction);
                if denominator > 0.0 {
                    data.color * denominator / (point.length() * normal.length())
                } else {
                    Color::black()
                }
            }
        }
    }
}

pub fn compute_lights(point: Vector3D, normal: Vector3D, lights: &[Light]) -> Color {
    lights.iter().fold(Color::black(), |acc, light| {
        acc + light.compute_color(point, normal)
    })
}
