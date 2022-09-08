use std::{fs::OpenOptions, io::Write, path::Path};

use crate::{color::Color, math::Vector3D, viewport::Viewport};

pub struct Canvas<const W: usize, const H: usize> {
    pub colors: Vec<Vec<Color>>,
}

impl<const W: usize, const H: usize> Canvas<W, H> {
    pub fn new() -> Self {
        Self {
            colors: vec![vec![Color::default(); W]; H],
        }
    }

    pub fn put_color(&mut self, row: usize, col: usize, color: Color) {
        self.colors[row][col] = color;
    }

    pub fn map_pixels<F>(&mut self, viewport: &Viewport, mut f: F)
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

    pub fn write_to_file(&self, output_dir: impl AsRef<Path>) {
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
                let r = (color.red * 255.0).round() as u8;
                let g = (color.green * 255.0).round() as u8;
                let b = (color.blue * 255.0).round() as u8;
                writeln!(file, "{} {} {}", r, g, b).unwrap();
            }
        }
    }
}
