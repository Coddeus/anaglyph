use image::GenericImageView;

extern crate image;

/// The possible Anaglyph colors listed [here](https://en.wikipedia.org/wiki/Anaglyph_3D#Anaglyphic_color_channels).
pub enum Coloring {
    RedGreen,
    RedBlue,
    RedCyan,
    Anachrome,
    Mirachrome,
    Trioscopic,
    Colorcode3D,
    MagentaCyan,
}

const PURERED: [f32 ; 3] = [255./255., 0./255., 0./255.];
const PUREGREEN: [f32 ; 3] = [0./255., 255./255., 0./255.];
const PUREBLUE: [f32 ; 3] = [0./255., 0./255., 255./255.];
const PURECYAN: [f32 ; 3] = [0./255., 255./255., 255./255.];
const DARKRED: [f32 ; 3] = [204./255., 0./255., 0./255.];
const CYAN: [f32 ; 3] = [153./255., 204./255., 255./255.];
const PUREMAGENTA: [f32 ; 3] = [255./255., 0./255., 255./255.];
const AMBER: [f32 ; 3] = [255./255., 191./255., 0./255.];
const DARKBLUE: [f32 ; 3] = [0./255., 0./255., 153./255.];

/// Creates the anaglyph from the given file, and saves it in the output file.
/// 
/// * `input_file` - The name of the input file.
/// * `output_file` - The name of the output file.
/// * `offset_x` - The horizontal difference between the two colors.
/// * `offset_y` - The vertical difference between the two colors.
/// * `coloring` - One of the anaglyph [Coloring] couples.
pub fn create(input_file: &str, output_file: &str, offset_x: isize, offset_y: isize, coloring: Coloring) -> () {
    let offset_x = offset_x /2; 
    let offset_y = offset_y /2;
    let direction: &str = {
        if offset_x>=0 {
            if offset_y>=0 { "bottomright" }
            else { "topright" }
        }
        else {
            if offset_y>=0 { "bottomleft" }
            else { "topleft" }
        }
    };
    let direction = direction.to_owned();
    let (color_left, color_right): (_, _) = match coloring {
        Coloring::RedGreen => (PURERED, PUREGREEN),
        Coloring::RedBlue => (PURERED, PUREBLUE),
        Coloring::RedCyan => (PURERED, PURECYAN),
        Coloring::Anachrome => (DARKRED, CYAN),
        Coloring::Mirachrome => (DARKRED, CYAN),
        Coloring::Trioscopic => (PUREGREEN, PUREMAGENTA),
        Coloring::Colorcode3D => (AMBER, DARKBLUE),
        Coloring::MagentaCyan => (PUREMAGENTA, PURECYAN),
    };
    let img = image::open(input_file).unwrap();
    let (in_x, in_y) = img.dimensions();
    let (in_x, in_y) = (in_x as isize, in_y as isize);
    let out_x = in_x - offset_x.abs();
    let out_y = in_y - offset_y.abs();
    let in_buf = img.to_rgb8().into_raw();
    let mut out_buf = vec![0u8 ; (out_x * out_y * 3) as usize];
    println!("Direction {}, In {}, Out {}", direction, in_x*in_y * 3, out_x * out_y * 3);

    let mut anaglyph = Anaglyph {
        in_buf,

        offset_x: offset_x.abs() as usize,
        in_x: in_x.abs() as usize,
        out_x: out_x.abs() as usize,
        
        offset_y: offset_y.abs() as usize,
        // in_y: in_y.abs() as usize,
        // out_y: out_y.abs() as usize,

        color_left,
        color_right,

        direction,
    };

    anaglyph.fill(&mut out_buf);

    image::save_buffer(&output_file, &out_buf, out_x as u32, out_y as u32, image::ColorType::Rgb8).unwrap();
}

struct Anaglyph {
    in_buf: Vec<u8>,

    offset_x: usize,
    in_x: usize,
    out_x: usize,

    // in_y: usize,
    offset_y: usize,
    // out_y: usize,

    color_left: [f32 ; 3],
    color_right: [f32 ; 3],

    direction: String,
}

impl Anaglyph {
    fn get(&self, i: usize) -> (u8, u8) {
        if self.direction==String::from("bottomright") {
            let val1 = self.in_buf[i + 3 * (self.offset_y*self.in_x + self.offset_x * (i/(self.out_x * 3) + 1))];
            let val2 = self.in_buf[i + 3 * (self.offset_x * i/(self.out_x * 3))];
            (val1, val2)
        }
        else if self.direction==String::from("topright") {
            let val1 = self.in_buf[i + 3 * (self.offset_x * (i/(self.out_x * 3) + 1))];
            let val2 = self.in_buf[i + 3 * (self.offset_y*self.in_x + self.offset_x * i/(self.out_x * 3))];
            (val1, val2)
        }
        else if self.direction==String::from("bottomleft") {
            let val1 = self.in_buf[i + 3 * (self.offset_y*self.in_x + self.offset_x * i/(self.out_x * 3))];
            let val2 = self.in_buf[i + 3 * (self.offset_x * (i/(self.out_x * 3) + 1))];
            (val1, val2)
        }
        else {
            let val1 = self.in_buf[i + 3 * (self.offset_x * i/(self.out_x * 3))];
            let val2 = self.in_buf[i + 3 * (self.offset_y*self.in_x + self.offset_x * (i/(self.out_x * 3) + 1))];
            (val1, val2)
        }
    }

    fn fill(&mut self, out_buf: &mut Vec<u8>) {
        let mut v1: u8;
        let mut v2: u8;
        for (i, byte) in out_buf.iter_mut().enumerate() {
            (v1, v2) = self.get(i);
            *byte = ((v1 as f32*self.color_left[i%3]) * 0.5 + (v2 as f32*self.color_right[i%3]) * 0.5) as u8;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs::{File, remove_file};

    use super::*;

    #[test]
    fn division() {
        assert!(9/5==1);
    }

    #[test]
    fn output_file_center() {
        let _ = remove_file("output_center.png");
        create("bao-menglong-unsplash.jpg", "output_center.png", 0, 0, Coloring::Colorcode3D);
        File::open("output_center.png").unwrap();
    }

    #[test]
    fn output_file_bottomright() {
        let _ = remove_file("output_bottomright.png");
        create("bao-menglong-unsplash.jpg", "output_bottomright.png", 100, 20, Coloring::RedCyan);
        File::open("output_bottomright.png").unwrap();
    }

    #[test]
    fn output_file_topright() {
        let _ = remove_file("output_topright.png");
        create("bao-menglong-unsplash.jpg", "output_topright.png", 70, -14, Coloring::RedGreen);
        File::open("output_topright.png").unwrap();
    }

    #[test]
    fn output_file_bottomleft() {
        let _ = remove_file("output_bottomleft.png");
        create("bao-menglong-unsplash.jpg", "output_bottomleft.png", -30, 8, Coloring::Anachrome);
        File::open("output_bottomleft.png").unwrap();
    }

    #[test]
    fn output_file_topleft() {
        let _ = remove_file("output_topleft.png");
        create("bao-menglong-unsplash.jpg", "output_topleft.png", -10, -2, Coloring::Trioscopic);
        File::open("output_topleft.png").unwrap();
    }
}
