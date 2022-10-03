mod utils;

use std::convert::TryInto;
use std::ops::Add;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::console::log;
use web_sys::{CanvasRenderingContext2d, ImageData};

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
//#[cfg(feature = "wee_alloc")]
//#[global_allocator]
//static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

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
    iterations: u32,
}

#[allow(dead_code)]
pub struct Pixel {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

fn palette(c: u32) -> Pixel {
    let normc = c as f64 / 1000.0;
    let b = normc * 255.0;
    let g = normc * 255.0;
    let r = normc * 255.0;
    Pixel {
        red: r as u8,
        green: g as u8,
        blue: b as u8,
        alpha: 255,
    }
    //vec![b as u8, g as u8, r as u8, 255]
    //pixels.push(b as u8);
    //pixels.push(g as u8);
    //pixels.push(r as u8);
    //pixels.push(255);
}

pub struct Plane {
    width: u32,
    height: u32,
    pixels: Vec<Pixel>,
    set: MandelbrotSet,
}

impl Plane {
    fn new() -> Plane {
        let width: u32 = 1000;
        let height: u32 = 1000;
        let set = MandelbrotSet {
            cxmin: -2.0,
            cxmax: 0.47,
            cymin: -1.12,
            cymax: 1.12,
            iterations: 1000,
        };
        let pixels = Vec::with_capacity((width * height).try_into().unwrap());
        Plane {
            width,
            height,
            pixels,
            set,
        }
    }

    fn calculate_set(&mut self) {
        let dy = (self.set.cymax - self.set.cymin) / self.height as f64;
        for i in 0..self.height {
            let cy = self.set.cymin + i as f64 * dy;
            let dx = (self.set.cxmax - self.set.cxmin) / self.width as f64;
            for j in 0..self.width {
                let cx = self.set.cxmin + j as f64 * dx;

                let c = Complex::new(cx, cy);
                let iters = self.compute_iterations(c);
                let pixel = palette(iters);
                self.pixels.push(pixel);
            }
        }
    }

    fn compute_iterations(&self, c: Complex) -> u32 {
        let mut z = Complex::new(0.0, 0.0);
        for i in 0..self.set.iterations {
            z = z.square() + c;
            if z.norm() > 2.0 {
                return i;
            }
        }
        self.set.iterations
    }

    //fn nbytes(&self) -> usize {
    //    // each pixel is 4 bytes, RBGA
    //    (self.width * self.height * 4).try_into().unwrap()
    //}
}

#[wasm_bindgen]
pub fn draw(ctx: &CanvasRenderingContext2d) -> Result<(), JsValue> {
    let mut plane = Plane::new();
    plane.calculate_set();
    //let pixels = plane.pixels;

    //let mut data_array: Vec<u8> = Vec::with_capacity(plane.nbytes());
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
