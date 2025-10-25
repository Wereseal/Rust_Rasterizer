use std::fs::File;
use std::io::*;
use std::time::Instant;

fn rotate_90(vec: [i32; 2]) -> [i32; 2]{
    [vec[1], -vec[0]]
}
fn dot_product(vec_one: [i32; 2], vec_two: [i32; 2]) -> i32{
    vec_one[0]*vec_two[0] + vec_one[1]*vec_two[1]
}
fn square(num: i32) -> i32{
    num*num
}
fn right_of_line(a: [i32; 2], b: [i32; 2], point: [i32; 2]) -> bool{
    dot_product([b[0]-a[0], b[1]-a[1]], rotate_90([point[0]-a[0], point[1]-a[1]])) >= 0
}

fn return_header(width: usize, height: usize) -> Vec<u8>{
    let mut header: Vec<u8> = vec!(0u8; 54);
    let filesize: u32 = (header.len() + (width * height)*3) as u32; 

    //header
    header[0..2].copy_from_slice(&[66, 77]); // BM 
    header[2..6].copy_from_slice(&filesize.to_le_bytes()); // size
    header[6..10].copy_from_slice(&[0, 0, 0, 0]); // unused(for now dun dun duuuu)
    header[10..14].copy_from_slice(&[54, 0, 0, 0]); // starting address of data

    // dib header
    header[14..18].copy_from_slice(&[40, 0, 0, 0]); // DIB header size
    header[18..22].copy_from_slice(&(width as u32).to_le_bytes()); // image width
    header[22..26].copy_from_slice(&(height as u32).to_le_bytes()); // image height
    header[26..28].copy_from_slice(&[1, 0]); // num of colour planes
    header[28..30].copy_from_slice(&[24, 0]); // bits per pixel
    header[30..34].copy_from_slice(&[0, 0, 0, 0]); // level of compression
    header[34..38].copy_from_slice(&[0, 0, 0, 0]); // image size(ignored due to no compression)
    header[38..42].copy_from_slice(&[35, 46, 0, 0]); // verticle resolution (not important)
    header[42..46].copy_from_slice(&[35, 46, 0, 0]); // horizontal resolution (not important)
    header[46..50].copy_from_slice(&[0, 0, 0, 0]); // colour palette (0 for default)
    header[50..54].copy_from_slice(&[0, 0, 0, 0]); // number of important colours (0 for all important)

    header
}
#[derive(Clone)]
struct Colour{
    red: u8,
    green: u8,
    blue: u8,
}
impl Colour{
    fn new(red: u8, green: u8, blue: u8) -> Colour{
        Colour{
            red,
            green,
            blue,
        }
    }
}
struct Frame{
    width: usize,
    height: usize,
    pixel_arr: Vec<Colour>,
}
impl Frame{
    fn output(&self, file: &mut File) -> std::io::Result<()>{
        let mut output_arr: Vec<u8> = Vec::new();
        output_arr.append(&mut return_header(self.width, self.height));
        let mut index: usize;
        let padding = self.width%4;
        for y in 0..self.height{
            for x in 0..self.width{
                index = y*self.width+x; 
                output_arr.push(self.pixel_arr[index].blue);
                output_arr.push(self.pixel_arr[index].green);
                output_arr.push(self.pixel_arr[index].red);
            }
            output_arr.append(&mut vec!(0u8; padding));
        }
        file.write_all(&output_arr);
        Ok(())
    }
    fn write_pixel(&mut self, x: usize, y: usize, colour: Colour){
        self.pixel_arr[y*self.width+x] = colour;
    }
    fn new(width: usize, height: usize) -> Frame{
        Frame{
            width,
            height,
            pixel_arr: vec!(Colour::new(0,0,0); width*height),
        }
    }
}
struct Triangle{
    a: [i32; 2],
    b: [i32; 2],
    c: [i32; 2],
    colour: Colour,
}

impl Triangle{
    fn new(a: [i32; 2], b: [i32; 2], c: [i32; 2], colour: Colour) -> Triangle{
        Triangle{
            a,
            b,
            c,
            colour,
        }
    }
    fn is_inside(&self, point: [i32; 2]) -> bool{
        right_of_line(self.a, self.b, point) && right_of_line(self.b, self.c, point) && right_of_line(self.c, self.a, point)
    }
    fn draw(&self, frame: &mut Frame){
        for y in 0..frame.height{
            for x in 0..frame.width{
                if(self.is_inside([x as i32,y as i32])){
                    frame.pixel_arr[y*frame.width+x] = self.colour.clone();
                }
            }
        }
    }
}
fn main() -> std::io::Result<()>{

    let now = Instant::now();
    let mut file = File::create("foo.bmp")?;
    let red = Colour::new(255, 255, 0);
    let mut frame = Frame::new(1000, 1000);
    let triangle = Triangle::new([200, 200], [800, 200], [500, 800], red);
    triangle.draw(&mut frame);
    frame.output(&mut file);
    println!("content took {}", now.elapsed().as_millis());
    Ok(())
}
