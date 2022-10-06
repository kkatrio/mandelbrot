#[allow(dead_code)]
const PI: f64 = std::f64::consts::PI;

use crate::Pixel;
use crate::ITER;

pub fn basic_palette(i: u32) -> Pixel {
    // https://rosettacode.org/wiki/Mandelbrot_set(BASIC256)
    let mut r = 0;
    let mut g = 0;
    let mut b = 0;
    if i < ITER {
        if i < 16 {
            r = i * 8;
            g = i * 8;
            b = 128 + i * 4;
        } else if i >= 16 && i < 64 {
            r = 128 + i - 16;
            g = 128 + i - 16;
            b = 192 + i - 16;
        } else if i >= 64 {
            r = 319 - i;
            g = (128 + r) / 2;
            b = r;
        }
    }
    Pixel {
        red: r as u8,
        green: g as u8,
        blue: b as u8,
        alpha: 255,
    }
}

fn hsv_px_palette(i: u32) -> Pixel {
    // https://rosettacode.org/wiki/Mandelbrot_set(C)
    let ratio = i as f64 / ITER as f64;
    let h: f64 = (4.0 * ratio) % 6.0;
    let c: f64 = 255.0;
    let x: f64 = c * (1.0 - ((h % 2.0) - 1.0).abs());
    //log!("h: {}, c : {}, x: {}", h, c, x,);

    let mut r: f64 = 0.0;
    let mut g: f64 = 0.0;
    let mut b: f64 = 0.0;
    match h as u32 {
        0 => {
            r = c;
            g = x;
        }
        1 => {
            r = x;
            g = c;
        }
        2 => {
            g = c;
            b = x;
        }
        3 => {
            g = x;
            b = c;
        }
        4 => {
            r = x;
            b = c;
        }
        5 => {
            r = c;
            b = x;
        }
        _ => {
            panic!("hue over 5");
        }
    }
    Pixel {
        red: r as u8,
        green: g as u8,
        blue: b as u8,
        alpha: 255,
    }
}

fn b1(v: f64) -> f64 {
    if v > 0.0031308 {
        v.powf(1.0 / 2.4) * 269.025 - 14.025
    } else {
        v * 3294.6
    }
}

fn b2(v: f64) -> f64 {
    if v > 0.2068965 {
        v * v * v
    } else {
        (v - 4.0 / 29.0) * (108.0 / 841.0)
    }
}

fn lch_palette(i: u32) -> Pixel {
    // https://en.wikipedia.org/wiki/Plotting_algorithms_for_the_Mandelbrot_set#LCH_Coloring
    let ratio = i as f64 / ITER as f64;

    let s = ratio;
    let v = 1.0 - (PI * s).powf(2.0);
    //v /= PI;

    let l = 75.0 - (75.0 * v);
    let c = 28.0 + l;
    let h = (360.0 * s).powf(1.15) % 360.0;

    // convert to RGB
    // https://gist.github.com/pushkine/c8ba98294233d32ab71b7e19a0ebdbb9
    let y = b2((l + 16.0) / 116.0);
    let x = b2((l + 16.0) / 116.0 + (c / 500.0) * (h * PI / 180.0).cos());
    let z = b2((l + 16.0) / 116.0 - (c / 200.0) * (h * PI / 180.0).sin());

    let r = b1(x * 3.021973625 - y * 1.617392459 - z * 0.404875592);
    let g = b1(x * -0.943766287 + y * 1.916279586 + z * 0.027607165);
    let b = b1(x * 0.069407491 - y * 0.22898585 + z * 1.159737864);

    Pixel {
        red: r as u8,
        green: g as u8,
        blue: b as u8,
        alpha: 255,
    }
}

fn palette(c: u32) -> Pixel {
    // https://en.wikipedia.org/wiki/Plotting_algorithms_for_the_Mandelbrot_set#Exponentially_mapped_and_Cyclic_Iterations
    let ratio = c as f64 / ITER as f64;
    let normc = (ratio * 360.0).powf(1.5) % 360.0;

    let b = normc;
    let g = normc;
    let r = normc;
    Pixel {
        red: r as u8,
        green: g as u8,
        blue: b as u8,
        alpha: 255,
    }
}
