import { Plane, draw } from "mandelbrot";

const width = 1000;
const height = 1000;

const canvas = document.getElementById("canvas");
canvas.width = width;
canvas.height = height;

const ctx = canvas.getContext("2d");
//draw(ctx, width, height);

// interactive
const basicColor = document.getElementById("basic");
basicColor.addEventListener("click", event => {
    draw(ctx, width, height, "basic");
});
const hsvColor = document.getElementById("hsv");
hsvColor.addEventListener("click", event => {
    draw(ctx, width, height, "hsv");
});
const lchColor = document.getElementById("lch");
lchColor.addEventListener("click", event => {
    draw(ctx, width, height, "lch");
});
const rgbColor = document.getElementById("rgb");
rgbColor.addEventListener("click", event => {
    draw(ctx, width, height, "rgb");
});
draw(ctx, width, height, "basic");
