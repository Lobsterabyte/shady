#[macro_use]
extern crate log;

use std::ops;

struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

fn main() {
    env_logger::init();
    let image_width = 256;
    let image_height = 256;

    print!("P3\n{} {}\n255\n", image_width, image_height);

    for i in 0..image_height {
        for j in 0..image_width {
            let red: f64 = i as f64 / image_height as f64;
            let green: f64 = j as f64 / image_width as f64;
            let blue: f64 = 0.5;

            let red_int: u8 = (red * 255.99) as u8;
            let green_int: u8 = (green * 255.99) as u8;
            let blue_int: u8 = (blue * 255.99) as u8;

            print!("{} {} {}\n", red_int, green_int, blue_int)
        }
        if (i + 1) % 16 == 0 {
            info!("Scanlines remaining: {}", 255 - i);
        }
    }
    info!("Complete");
}
