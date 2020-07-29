#![no_std]

use vga::colors::Color16;
use vga::writers::Graphics640x480x16;
use vga_figures::figures2d::Figures2D;

const WIDTH: isize = 640;
const HEIGHT: isize = 480;

pub fn draw(figures: Figures2D<Graphics640x480x16>, color: Color16) {
    let x = WIDTH / 2;
    let y = HEIGHT / 2;
    let ab = y - 50;
    figures.ellipse(x, y, ab, ab, color);

    let arr = [
        160, 240, 200, 260, 160, 280, 220, 300, 180, 320, 240, 340, 200, 360, 260, 380, 340, 420,
        420, 360, 420, 290, 490, 250, 490, 230, 480, 235, 470, 220, 465, 240, 415, 265, 425, 220,
        445, 210, 455, 220, 460, 205, 490, 190, 475, 175, 475, 145, 445, 165, 380, 185, 330, 165,
        290, 165, 250, 120, 250, 185, 200, 140, 210, 215,
    ];

    figures.polygon(&arr, color);

    figures.text((x - 40) as usize, y as usize, "CipherDogs", color);
    figures.text(
        (x - 70) as usize,
        (y + 16) as usize,
        "Cyber-crypto team",
        color,
    );
}
