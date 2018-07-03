use rand;
use rand::Rng;
use imageproc::drawing::*;
use image::{Rgba, RgbaImage, DynamicImage};
use imageproc::rect::Rect;
use std::f32;

pub struct DNA{
    //[(R, G, B, alpha, x, y, radius), ...]
    gene: [(u8, u8, u8, u8, i32, i32, i32); 100],
    fitness: f32,
}

impl DNA{
    pub fn new() -> DNA{
        let mut dna = DNA{
            gene: [(0, 0, 0, 0, 0, 0, 0); 100],
            fitness: 0.01
        };
        let mut rng = rand::thread_rng();
        for i in 0..dna.len(){
            //RGB
            dna.gene[i].0 = rng.gen_range(0, 256);
            dna.gene[i].1 = rng.gen_range(0, 256);
            dna.gene[i].2 = rng.gen_range(0, 256);
            //alpha
            dna.gene[i].3 = rng.gen_range(0, 256);
            //x,y
            dna.gene[i].4 = rng.gen_range(0, 300);
            dna.gene[i].5 = rng.gen_range(0, 300);
            // radius value
            dna.gene[i].6 = rng.gen_range(0, 150);
        }
        dna
    }

    pub fn calc_fitness(&mut self, width:u32, height:u32, target: RgbaImage){
        //绘图
        let mut image = DynamicImage::new_rgba8(width, height);
        draw_filled_rect_mut(&mut image, Rect::at(0, 0).of_size(width, height), Rgba([255, 255, 255, 255]));
        for ch in self.gene.iter(){
            let diameter = ch.6*2;
            draw_filled_ellipse(&mut image, (ch.4, ch.5), diameter, diameter, Rgba([ch.0, ch.1, ch.2, ch.3]));
        }
        let test_buffer = image.to_rgba();
        let mut test_pixels = test_buffer.pixels();
        let mut target_pixels = target.pixels();
        while let Some(pixel) = test_pixels.next(){
            let target_pixel = target_pixels.next().unwrap();
            //获取每隔颜色的差值
            let dr = target_pixel[0] - pixel[0];
            let dg = target_pixel[1] - pixel[1];
            let db = target_pixel[2] - pixel[2];
            let da = target_pixel[3] - pixel[3];
            //计算颜色之间的3D空间距离
            let pixel_fitness = dr as f32 * dr as f32
                                 + dg as f32 * dg as f32 
                                 + db as f32 * db as f32;
            self.fitness += pixel_fitness;
        }
        self.fitness = 1.0/self.fitness.log(f32::consts::E);
    }

    pub fn len(&self) -> usize{
        self.gene.len()
    }
}