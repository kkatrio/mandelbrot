mod palettes;
#[macro_use]
mod utils;

use crate::palettes::{BasicColoring, HsvColoring, LchColoring, Palette, RgbNormalizedColoring};
use std::ops::Add;
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};

#[derive(Clone, Copy)]
struct Complex {
    re: f64,
    im: f64,
}

impl Complex {
    fn new(re: f64, im: f64) -> Complex {
        Complex { re, im }
    }

    fn square(self) -> Complex {
        Complex {
            re: self.re * self.re - self.im * self.im,
            im: 2.0 * self.re * self.im,
        }
    }

    fn norm(&self) -> f64 {
        self.re * self.re + self.im * self.im
    }
}

impl Add<Complex> for Complex {
    type Output = Complex;
    fn add(self, other: Complex) -> Self::Output {
        Self {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }
}

struct MandelbrotSet {
    cxmin: f64,
    cxmax: f64,
    cymin: f64,
    cymax: f64,
    max_iters: u32,
}

#[allow(dead_code)]
pub struct Pixel {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

pub struct Plane {
    width: u32,
    height: u32,
    pixels: Vec<Pixel>,
    set: MandelbrotSet,
}

impl Plane {
    fn new(
        width: u32,
        height: u32,
        xmin: f64,
        xmax: f64,
        ymin: f64,
        ymax: f64,
        max_iters: u32,
    ) -> Plane {
        let set = MandelbrotSet {
            cxmin: xmin,
            cxmax: xmax,
            cymin: ymin,
            cymax: ymax,
            max_iters,
        };
        let pixels = Vec::new();
        Plane {
            width,
            height,
            pixels,
            set,
        }
    }

    fn calculate_set(&mut self, pal: &Box<dyn Palette>) {
        let dy = (self.set.cymax - self.set.cymin) / self.height as f64;
        for i in 0..self.height {
            let cy = self.set.cymin + i as f64 * dy;
            let dx = (self.set.cxmax - self.set.cxmin) / self.width as f64;
            for j in 0..self.width {
                let cx = self.set.cxmin + j as f64 * dx;

                let c = Complex::new(cx, cy);
                let iters = self.compute_iterations(c);
                let pixel = pal.color(iters, self.set.max_iters);
                self.pixels.push(pixel);
            }
        }
    }

    fn compute_iterations(&self, c: Complex) -> u32 {
        let mut z = Complex::new(0.0, 0.0);
        for i in 0..self.set.max_iters {
            z = z.square() + c;
            if z.norm() > 2.0 {
                return i;
            }
        }
        self.set.max_iters
    }
}

// TODO: use Rc and callbacks https://github.com/rustwasm/wasm-bindgen/blob/main/examples/paint/src/lib.rs
// TODO: use wasm_bindgen(start)
#[wasm_bindgen]
pub fn draw(
    ctx: &CanvasRenderingContext2d,
    w: u32,
    h: u32,
    palette: Option<String>,
    xmin: f64,
    xmax: f64,
    ymin: f64,
    ymax: f64,
    iters: u32,
) -> Result<(), JsValue> {
    set_panic_hook();
    let mut plane = Plane::new(w, h, xmin, xmax, ymin, ymax, iters);

    let palette: Box<dyn Palette> = match palette.unwrap().as_ref() {
        "rgb" => Box::new(RgbNormalizedColoring),
        "basic" => Box::new(BasicColoring),
        "hsv" => Box::new(HsvColoring),
        "lch" => Box::new(LchColoring),
        _ => panic!("Unknown palette!"),
    };
    plane.calculate_set(&palette);

    let mut data_array: Vec<u8> = Vec::new();
    plane.pixels.iter().for_each(|p| {
        data_array.push(p.red);
        data_array.push(p.green);
        data_array.push(p.blue);
        data_array.push(p.alpha);
    });

    let image = ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(&data_array),
        plane.width,
        plane.height,
    )?;

    ctx.put_image_data(&image, 0.0, 0.0)
}
