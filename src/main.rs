use std::{fs::OpenOptions, io::Write, path::Path};

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

// TODO: t_min and t_max?
struct Ray {
    point: Point3D,
    dir: Vector3D,
}

struct Camera {
    point: Point3D,
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

struct Time {
    t: f32,
}

enum Intersection {
    None,
    One(Time),
    Two((Time, Time)),
}

fn sphere_ray_intersection(sphere: &Sphere, ray: &Ray) -> Intersection {
    // TODO: Hook this up
    Intersection::None
}

fn get_color(viewport_x: i32, viewport_y: i32, _scene: &[Sphere]) -> Color {
    let mut color = Color {
        red: 0,
        green: 0,
        blue: 0,
    };
    if viewport_x % 100 < 50 {
        color.red += 127;
        color.green += 127;
        color.blue += 127;
    }
    if viewport_y % 100 < 50 {
        color.red += 127;
        color.green += 127;
        color.blue += 127;
    }
    color
}

struct Canvas<const W: usize, const H: usize> {
    colors: [[Color; W]; H],
}

impl<const W: usize, const H: usize> Canvas<W, H> {
    fn new() -> Self {
        Self {
            colors: [[Color::default(); W]; H],
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
        point: Point3D {
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

    for v_y in -(VIEWPORT_HEIGHT as i32 / 2)..VIEWPORT_HEIGHT as i32 / 2 {
        for v_x in -(VIEWPORT_WIDTH as i32 / 2)..VIEWPORT_WIDTH as i32 / 2 {
            let color = get_color(v_x, v_y, &scene);
            canvas.put_color(v_x, v_y, color);
        }
    }
    canvas.write_to_file("output.ppm");
}
