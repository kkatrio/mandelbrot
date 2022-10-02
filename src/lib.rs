mod utils;

use num_complex::Complex;
use std::convert::TryInto;
use wasm_bindgen::prelude::*;

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
}

#[wasm_bindgen]
pub struct Plane {
    width: u32,
    height: u32,
    pixels: Vec<Pixel>,
    set: MandelbrotSet,
}

#[wasm_bindgen]
impl Plane {
    pub fn new() -> Plane {
        let width = 1000;
        let height = 1000;
        let pixels = (0..height * width)
            .map(|_| Pixel {
                red: 0,
                green: 0,
                blue: 0,
                alpha: 0,
            })
            .collect();

        let set = MandelbrotSet {
            cxmin: -2.0,
            cxmax: 0.47,
            cymin: -1.12,
            cymax: 1.12,
            iterations: 1000,
        };
        Plane {
            width,
            height,
            pixels,
            set,
        }
    }

    pub fn npixels(&self) -> u32 {
        self.width * self.height
    }

    pub fn pixels_ptr(&self) -> *const Pixel {
        self.pixels.as_ptr()
    }

    pub fn calculate_set(&mut self) {
        let mut pixels: Vec<Pixel> =
            Vec::with_capacity((self.width * self.height).try_into().unwrap());
        let dy = (self.set.cymax - self.set.cymin) / self.height as f64;
        for i in 0..self.height {
            let cy = self.set.cymin + i as f64 * dy;
            let dx = (self.set.cxmax - self.set.cxmin) / self.width as f64;
            for j in 0..self.width {
                let cx = self.set.cxmin + j as f64 * dx;

                let c = Complex::new(cx, cy);
                let iters = self.compute_iterations(&c);
                let pixel = palette(iters);
                pixels.push(pixel);
            }
        }

        self.pixels = pixels;
    }

    fn compute_iterations(&self, c: &Complex<f64>) -> u32 {
        let mut z = Complex::new(0.0, 0.0);
        for i in 0..self.set.iterations {
            z = z * z + c;
            if z.norm() > 2.0 {
                return i;
            }
        }
        self.set.iterations
    }
}
