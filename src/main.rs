use std::{fs::OpenOptions, io::Write, mem::Discriminant, path::Path};

const SCREEN_WIDTH: usize = 700;
const SCREEN_HEIGHT: usize = 700;

#[derive(Default, Debug, Clone, Copy)]
struct Vector3D {
    x: f32,
    y: f32,
    z: f32,
}

fn dot(lhs: Vector3D, rhs: Vector3D) -> f32 {
    lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
}

struct Viewport {
    width: f32,
    height: f32,
    center: Vector3D,
}

// TODO: t_min and t_max?
struct Ray {
    origin: Vector3D,
    dir: Vector3D,
}

struct Camera {
    origin: Vector3D,
    dir: Vector3D,
}

#[derive(Default, Debug, Clone, Copy)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

struct Sphere {
    center: Vector3D,
    radius: f32,
    color: Color,
}

fn sphere_ray_intersection(sphere: &Sphere, ray: &Ray) -> Option<(f32, f32)> {
    let co = Vector3D {
        x: ray.origin.x - sphere.center.x,
        y: ray.origin.y - sphere.center.y,
        z: ray.origin.z - sphere.center.z,
    };

    let a = dot(ray.dir, ray.dir);
    let b = 2.0 * dot(ray.dir, co);
    let c = dot(co, co) - sphere.radius * sphere.radius;

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

fn trace_ray(ray: Ray, scene: &[Sphere]) -> Color {
    let background_color = Color {
        red: 175,
        green: 175,
        blue: 175,
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
    closest_sphere.map_or(background_color, |sphere| sphere.color)
}

struct Canvas<const W: usize, const H: usize> {
    colors: Vec<Vec<Color>>,
}

impl<const W: usize, const H: usize> Canvas<W, H> {
    fn new() -> Self {
        Self {
            colors: vec![vec![Color::default(); W]; H],
        }
    }

    pub fn put_color(&mut self, row: usize, col: usize, color: Color) {
        self.colors[row][col] = color;
    }

    fn map_pixels<F>(&mut self, viewport: &Viewport, mut f: F)
    where
        F: FnMut(Vector3D) -> Color,
    {
        for row in 0..H {
            for col in 0..W {
                let view_x = col as f32 * (viewport.width / W as f32) - viewport.width / 2.0;
                let view_y = -(row as f32 * (viewport.height / W as f32) - viewport.height / 2.0);
                let color = f(Vector3D {
                    x: view_x,
                    y: view_y,
                    z: viewport.center.z,
                });
                self.put_color(row, col, color);
            }
        }
    }

    fn write_to_file(&self, output_dir: impl AsRef<Path>) {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(output_dir.as_ref())
            .expect("Could not create output file");
        writeln!(file, "P3").unwrap();
        writeln!(file, "{} {}", W, H).unwrap();
        writeln!(file, "255").unwrap();
        for row in 0..H {
            for col in 0..W {
                let color = self.colors[row][col];
                writeln!(file, "{} {} {}", color.red, color.blue, color.green).unwrap();
            }
        }
    }
}

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
                red: 255,
                green: 0,
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
                red: 255,
                green: 0,
                blue: 0,
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
