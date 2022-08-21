use std::{fs::OpenOptions, io::Write, mem::Discriminant, path::Path};

// Canvas has origin at the center, with positive x to the right and positive y up
const CANVAS_WIDTH: usize = 700;
const CANVAS_HEIGHT: usize = 700;

// Viewport had origin on the top-left, with potiive x to the right and positive y down
const VIEWPORT_WIDTH: usize = 700;
const VIEWPORT_HEIGHT: usize = 700;

fn viewport_to_canvas(x: i32, y: i32) -> Option<(usize, usize)> {
    let canvas_width_scale = CANVAS_WIDTH as f32 / VIEWPORT_WIDTH as f32;
    let canvas_height_scale = CANVAS_HEIGHT as f32 / VIEWPORT_HEIGHT as f32;
    let canvas_point_x = (x as f32 + VIEWPORT_WIDTH as f32 / 2.0) * canvas_width_scale;
    let canvas_point_y = (CANVAS_HEIGHT - 1) as f32
        - (y as f32 + VIEWPORT_HEIGHT as f32 / 2.0) * canvas_height_scale;
    if canvas_point_x >= 0.0
        && canvas_point_x <= CANVAS_WIDTH as f32
        && canvas_point_y >= 0.0
        && canvas_point_y <= CANVAS_HEIGHT as f32
    {
        Some((
            canvas_point_x.round() as usize,
            canvas_point_y.round() as usize,
        ))
    } else {
        print!(
            "invalid point: V[{},{}], C[{},{}]",
            x, y, canvas_point_x, canvas_point_y
        );
        None
    }
}

#[derive(Default, Debug, Clone, Copy)]
struct Point3D {
    x: f32,
    y: f32,
    z: f32,
}

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
    width: usize,
    height: usize,
    center: Point3D,
}

impl Viewport {
    fn map_points<F>(&self, mut f: F)
    where
        F: FnMut(Point3D),
    {
        for w_y in -(self.height as i32 / 2)..self.height as i32 / 2 {
            for w_x in -(self.width as i32 / 2)..self.width as i32 / 2 {
                f(Point3D {
                    x: self.center.x + w_x as f32,
                    y: self.center.y + w_y as f32,
                    z: self.center.z,
                })
            }
        }
    }
}

// TODO: t_min and t_max?
struct Ray {
    origin: Point3D,
    dir: Vector3D,
}

struct Camera {
    origin: Point3D,
    dir: Vector3D,
}

#[derive(Default, Debug, Clone, Copy)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

struct Sphere {
    center: Point3D,
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

    pub fn put_color(&mut self, row: i32, col: i32, color: Color) {
        let (c_row, c_col) = viewport_to_canvas(row, col).unwrap_or_else(|| {
            panic!(
                "Attempted to sample point outside of canvas range. Canvas[{}, {}] Point[{},{}]",
                W, H, row, col
            )
        });
        self.colors[c_row][c_col] = color;
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
        origin: Point3D {
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
        width: VIEWPORT_WIDTH,
        height: VIEWPORT_HEIGHT,
        center: Point3D {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
    };
    let scene = [
        Sphere {
            center: Point3D {
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
            center: Point3D {
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
            center: Point3D {
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

    let mut canvas = Canvas::<CANVAS_WIDTH, CANVAS_HEIGHT>::new();
    viewport.map_points(|point| {
        let ray = Ray {
            origin: camera.origin,
            dir: Vector3D {
                x: point.x as f32,
                y: point.y as f32,
                z: point.z as f32,
            },
        };
        let color = trace_ray(ray, &scene);
        canvas.put_color(point.x as i32, point.y as i32, color);
    });

    canvas.write_to_file("output.ppm");
}
