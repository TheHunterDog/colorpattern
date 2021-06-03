extern crate image;
extern crate num;

use image::png::PNGEncoder;
use image::ColorType;
use num::ToPrimitive;
use rgb::*;
use core::f32;
use std::fs::File;
use std::io::Write;
use std::usize;


struct Image{
  width: usize,
  height:usize
}
fn main() {
    println!("Hello, world!");

    let args: Vec<String> = std::env::args().collect();
    if args.len() != 1 {
        writeln!(std::io::stderr(), "You need to give us a color").unwrap();
        std::process::exit(1)
    }
    let baseColor = RGB8 {
        r: 0,
        g: 100,
        b: 255,
    };
    let reset = 10;
    let darken = 1.5;
    let image = Image{width:200,height:200};
    let mut pixels = vec![0;image.width*image.height];
    render(&mut pixels, &image,reset,darken);
    write_image(&pixels,&image).expect("Error has occurd while saving the file");
}

fn render(pixels:&mut [u8],image:&Image,reset:u16,darkenfactor:f32){
  let mut it:u16 = 0;
  for row in 0..image.width{
    for colluumn in 0..image.height{
      it += 1;
      if it == reset {
        it = 0
      }

      pixels[row*image.height+colluumn] = (it as f32 * darkenfactor) as u8;
    }
  }
}

fn write_image(pixels:&[u8],image:&Image) -> Result<(),std::io::Error>{
  let file = File::create("Collorpattern.png")?;
  let enc:PNGEncoder<File> = PNGEncoder::new(file);
  enc.encode(pixels, image.width.to_u32().unwrap(), image.height.to_u32().unwrap(), ColorType::Gray(4))?;

  Ok(())
}