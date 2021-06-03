extern crate image;
extern crate num;

use core::f32;
use image::png::PNGEncoder;
use image::ColorType;
use image::{ImageBuffer, Pixel, Rgb, Rgba, RgbaImage};
use num::ToPrimitive;
use rgb::*;
use std::fs::File;
use std::io::Write;
use std::time::{Duration, Instant};
use std::u32;
use std::usize;
struct Image {
    width: usize,
    height: usize,
}
fn main() {
    let time = Instant::now();
    println!("Generating please wait!");
    for poss in 0..=100 {
        for darken in 0..100 {
            let baseColor = Rgb([255, 255, 255]);
            let reset: u8 = poss;
            ///minimal 0.5
            let darken: f32 = 0.2+darken as f32;
            let image = Image {
                width: 200,
                height: 200,
            };
            let mut img =
                ImageBuffer::<Rgb<u8>, Vec<u8>>::new(image.width as u32, image.height as u32);
            render(&mut img, baseColor, reset, darken);
            img.save(format!("output-reset:{}-Darken:{}.png", poss,darken)).unwrap();
        }
    }

    println!("Finished in {}seconds", time.elapsed().as_secs())
}

fn render(
    image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    baseColor: Rgb<u8>,
    reset: u8,
    darkenfactor: f32,
) {
    let mut iter = 0;
    for x in 0..image.width() {
        for y in 0..image.height() {
            iter += 1;
            if iter as u8 == reset {
                iter = 0
            }
            println!("Calculating pixel {} {}", x, y);
            image.put_pixel(
                x,
                y,
                calculate_rgb_value(baseColor, iter, reset, darkenfactor),
            )
        }
    }
}

fn calculate_rgb_value(
    baseColor: Rgb<u8>,
    iteration: u32,
    reset: u8,
    darkenfactor: f32,
) -> Rgb<u8> {
    let mut color = baseColor.clone();

    if iteration == 0 {
        return color;
    }
    color = baseColor.map(|color| (color as f32 * darkenfactor) as u8);
    color
}
