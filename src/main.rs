#[macro_use]
extern crate log;

pub mod vec3;
use vec3::{cross, Vec3};

fn main() {
    env_logger::init();
    let image_width = 256;
    let image_height = 256;

    let test = Vec3::new(1.0, 2.0, 2.0);
    let test2 = Vec3::new(2.0, 3.0, 7.0);

    //TODO: make it delete ../output/image.ppm then write to it line by line

    /*print!("P3\n{} {}\n255\n", image_width, image_height);

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
    info!("Complete");*/

    println!("|(1,2,2)| = {}", { test.length() });
    println!("(1,2,2)x(2,3,7) = {:?}", { cross(test, test2) });
}
