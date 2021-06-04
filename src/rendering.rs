use crate::{AllData, Params, PixelColor};
use cairo::{Format, ImageSurface};

#[allow(clippy::many_single_char_names)]
fn hsv(h: f64, s: f64, v: f64) -> (u8, u8, u8) {
    let c = v * s;
    let h = if h % 360.0 < 0.0 {
        h % 360.0 + 360.0
    } else {
        h % 360.0
    };
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;
    let (rp, gp, bp) = if h >= 0.0 && h < 60.0 {
        (c, x, 0.0)
    } else if h >= 60.0 && h < 120.0 {
        (x, c, 0.0)
    } else if h >= 120.0 && h < 180.0 {
        (0.0, c, x)
    } else if h >= 180.0 && h < 240.0 {
        (0.0, x, c)
    } else if h >= 240.0 && h < 300.0 {
        (x, 0.0, c)
    } else if h >= 300.0 && h < 360.0 {
        (c, 0.0, x)
    } else {
        unreachable!();
    };

    (
        ((rp + m) * 255.0) as u8,
        ((gp + m) * 255.0) as u8,
        ((bp + m) * 255.0) as u8,
    )
}

fn color_from_elev_dist(params: &Params, elev: f64, dist: f64) -> (u8, u8, u8) {
    let dist_ratio = dist / params.view.frame.max_distance;
    if elev <= params.view.coloring.water_level() {
        let mul = 1.0 - dist_ratio * 0.6;
        (0, (128.0 * mul) as u8, (255.0 * mul) as u8)
    } else {
        let elev_ratio = elev / 4500.0;
        let h = 120.0
            - 240.0
                * if elev_ratio < 0.0 {
                    -(-elev_ratio).powf(0.65)
                } else {
                    elev_ratio.powf(0.65)
                };
        let v = if elev_ratio > 0.7 {
            2.1 - elev_ratio * 2.0
        } else {
            0.9 - elev_ratio / 0.7 * 0.2
        } * (1.0 - dist_ratio * 0.6);
        let s = 1.0 - dist_ratio * 0.9;
        hsv(h, s, v)
    }
}

pub fn create_surface(data: &AllData) -> ImageSurface {
    let width = data.params.output.width as usize;
    let height = data.params.output.height as usize;
    let mut img = ImageSurface::create(Format::Rgb24, width as i32, height as i32)
        .expect("couldn't create a surface");
    {
        let mut img_data = img.get_data().expect("couldn't get surface data");

        for y in 0..height {
            for x in 0..width {
                let result = &data.result[y][x];
                let color = if let Some(result) = result.get(0) {
                    match result.color {
                        PixelColor::Terrain => {
                            color_from_elev_dist(&data.params, result.elevation, result.distance)
                        }
                        PixelColor::Rgb(color) => (
                            (color.r * 255.0) as u8,
                            (color.g * 255.0) as u8,
                            (color.b * 255.0) as u8,
                        ),
                    }
                } else {
                    (28, 28, 28)
                };
                let pixel = ((y * width) + x) * 4;
                img_data[pixel] = color.2;
                img_data[pixel + 1] = color.1;
                img_data[pixel + 2] = color.0;
                img_data[pixel + 3] = 0;
            }
        }
    }

    img
}
