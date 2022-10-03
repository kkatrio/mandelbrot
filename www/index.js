import { memory } from "mandelbrot/mandelbrot_bg";
import { Plane, draw } from "mandelbrot";


//const plane = Plane.new();
//plane.calculate_set();
//
//const pixelsPtr = plane.pixels_ptr();
//const npixels = plane.npixels();
//// each pixel is 4 bytes, RGBA
//const nbytes = npixels * 4;
//const dataArray = new Uint8ClampedArray(memory.buffer, pixelsPtr, nbytes);
//
//const length = Math.sqrt(npixels);
//console.log("pixels: ", npixels, "length: ", length);
//
const width = 1000;
const height = 1000;

const canvas = document.getElementById("canvas");
canvas.width = width;
canvas.height = height;

const ctx = canvas.getContext("2d");
draw(ctx, width, height);

//const image = new ImageData(dataArray, length, length);
//ctx.putImageData(image, 0, 0);
