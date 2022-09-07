mod camera;
mod canvas;
mod color;
mod math;
mod ray;
mod sphere;
mod viewport;

use camera::Camera;
use canvas::Canvas;
use color::Color;
use math::Vector3D;
use ray::{trace_ray, Ray};
use sphere::Sphere;
use viewport::Viewport;

const SCREEN_WIDTH: usize = 700;
const SCREEN_HEIGHT: usize = 700;

fn main() {
    let camera = Camera {
        origin: Vector3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        dir: Vector3D {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
    };
    let viewport = Viewport {
        width: 1.0,
        height: 1.0,
        center: Vector3D {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
    };
    let scene = [
        Sphere {
            center: Vector3D {
                x: 0.0,
                y: -1.0,
                z: 3.0,
            },
            radius: 1.0,
            color: Color {
                red: 255,
                green: 0,
                blue: 0,
            },
        },
        Sphere {
            center: Vector3D {
                x: 2.0,
                y: 0.0,
                z: 4.0,
            },
            radius: 1.0,
            color: Color {
                red: 0,
                green: 255,
                blue: 0,
            },
        },
        Sphere {
            center: Vector3D {
                x: -2.0,
                y: 0.0,
                z: 4.0,
            },
            radius: 1.0,
            color: Color {
                red: 0,
                green: 0,
                blue: 255,
            },
        },
    ];

    let mut canvas = Canvas::<SCREEN_WIDTH, SCREEN_HEIGHT>::new();
    canvas.map_pixels(&viewport, |point| {
        let ray = Ray {
            origin: camera.origin,
            dir: point,
        };
        trace_ray(ray, &scene)
    });

    canvas.write_to_file("output.ppm");
}
