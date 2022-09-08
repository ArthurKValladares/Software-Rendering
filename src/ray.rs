use crate::{
    color::Color,
    light::{compute_lights, Light},
    math::Vector3D,
    sphere::Sphere,
};

// TODO: t_min and t_max?
pub struct Ray {
    pub origin: Vector3D,
    pub dir: Vector3D,
}

pub fn sphere_ray_intersection(sphere: &Sphere, ray: &Ray) -> Option<(f32, f32)> {
    let co = Vector3D {
        x: ray.origin.x - sphere.center.x,
        y: ray.origin.y - sphere.center.y,
        z: ray.origin.z - sphere.center.z,
    };

    let a = ray.dir.dot(ray.dir);
    let b = 2.0 * ray.dir.dot(co);
    let c = co.dot(co) - sphere.radius * sphere.radius;

    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        None
    } else {
        let discriminant_root = discriminant.sqrt();
        let t0 = (-b - discriminant_root) / (2.0 * a);
        let t1 = (-b + discriminant_root) / (2.0 * a);
        Some((t0, t1))
    }
}

pub fn trace_ray(ray: Ray, scene: &[Sphere], ligths: &[Light]) -> Color {
    let background_color = Color {
        red: 0.8,
        green: 0.8,
        blue: 0.8,
    };
    let mut closest_t = None;
    let mut closest_sphere = None;
    for sphere in scene {
        let intersection = sphere_ray_intersection(sphere, &ray);
        match intersection {
            Some((t0, t1)) => {
                let min_time = t0.min(t1);
                if closest_t.is_none() || min_time < closest_t.unwrap() {
                    closest_t = Some(min_time);
                    closest_sphere = Some(sphere);
                }
            }
            None => {}
        };
    }
    closest_sphere.map_or(background_color, |sphere| {
        let point = ray.origin + ray.dir * closest_t.unwrap();
        let normal = (point - sphere.center).normalize();
        let lights = compute_lights(point, normal, ligths);
        sphere.color * lights
    })
}
