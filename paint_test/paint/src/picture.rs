use rand;
use rand::Rng;
use imageproc::drawing::*;
use image::{Rgba, RgbaImage, DynamicImage};
use imageproc::rect::Rect;
use std::f32;
use std::clone::Clone;
use rand::thread_rng;
use std::time::{Duration, Instant};
use ::duration_to_milis;

#[derive(Copy)]
pub struct Picture{
    //[(R, G, B, alpha, x, y, radius), ...]
    gene: [(u8, u8, u8, u8, i32, i32, i32); 100],
    pub fitness: f32,
}

impl Clone for Picture {
    fn clone(&self) -> Picture { *self }
}

impl Picture{
    pub fn new() -> Picture{
        let mut dna = Picture{
            gene: [(0, 0, 0, 0, 0, 0, 0); 100],
            fitness: 0.01
        };
        let mut rng = thread_rng();
        for i in 0..dna.len(){
            //RGB
            dna.gene[i].0 = rng.gen_range(0, 256) as u8;
            dna.gene[i].1 = rng.gen_range(0, 256) as u8;
            dna.gene[i].2 = rng.gen_range(0, 256) as u8;
            //alpha
            dna.gene[i].3 = rng.gen_range(0, 256) as u8;
            //x,y
            dna.gene[i].4 = rng.gen_range(0, 300);
            dna.gene[i].5 = rng.gen_range(0, 300);
            // radius value
            dna.gene[i].6 = rng.gen_range(0, 150);
        }
        dna
    }

    //绘图
    pub fn render(&mut self, width:u32, height:u32) -> DynamicImage{
        //let now = Instant::now();
        let mut image = DynamicImage::new_rgba8(width, height);
        draw_filled_rect_mut(&mut image, Rect::at(0, 0).of_size(width, height), Rgba([255, 255, 255, 255]));
        for ch in self.gene.iter_mut(){
            //let diameter = ch.6*2;
            if ch.6 == 0{
                ch.6 = 1;
            }
            let rect = Rect::at(ch.4, ch.5).of_size(ch.6 as u32, ch.6 as u32);
            draw_filled_rect_mut(&mut image, rect, Rgba([ch.0, ch.1, ch.2, ch.3]));
            //draw_filled_circle_mut(&mut image, (ch.4, ch.5), ch.6, Rgba([ch.0, ch.1, ch.2, ch.3]));
        }
        //info!("render 耗时:{}ms", duration_to_milis(&now.elapsed()));
        image
    }

    pub fn calc_fitness(&mut self, target: &RgbaImage){
        let (width, height) = (target.width(), target.height());
        
        let image = self.render(width, height);
        let test_buffer = image.to_rgba();
        let mut test_pixels = test_buffer.pixels();
        let mut target_pixels = target.pixels();
        while let Some(pixel) = test_pixels.next(){
            let target_pixel = target_pixels.next().unwrap();
            //获取每隔颜色的差值
            let dr = target_pixel[0] as f32 - pixel[0] as f32;
            let dg = target_pixel[1] as f32 - pixel[1] as f32;
            let db = target_pixel[2] as f32 - pixel[2] as f32;
            let da = target_pixel[3] as f32 - pixel[3] as f32;
            //计算颜色之间的3D空间距离
            let pixel_fitness = dr * dr + dg * dg + db * db + da * da;
            self.fitness += pixel_fitness;
        }
        self.fitness = 1.0/self.fitness.log(f32::consts::E);
    }

    //杂交(单点)
    pub fn crossover(&self, crossover_rate:f32, dad:&Picture) -> (Picture, Picture) {
        let mum = self;
        let mut rng = thread_rng();
        if rng.gen::<f32>()>crossover_rate {
            (*mum, *dad)
        }else{
            let mut baby1 = mum.gene.clone();
            let mut baby2 = dad.gene.clone();
            //确定杂交点
            let cp = rng.gen_range(0, mum.gene.len());
            for i in cp..self.gene.len(){
                baby1[i] = dad.gene[i];
                baby2[i] = mum.gene[i];
            }
            (Picture{
                gene: baby1,
                fitness: 0.01
            },
            Picture{
                gene: baby2,
                fitness: 0.01
            })
        }
    }

    pub fn mutate(&mut self, mutation_rate:f32){
        let mut rng = rand::thread_rng();
        for i in 0..self.gene.len(){
            if rng.gen::<f32>() < mutation_rate{
                //RGB
                self.gene[i].0 = rng.gen_range(0, 256) as u8;
                self.gene[i].1 = rng.gen_range(0, 256) as u8;
                self.gene[i].2 = rng.gen_range(0, 256) as u8;
                //alpha
                self.gene[i].3 = rng.gen_range(0, 256) as u8;
                //x,y
                self.gene[i].4 = rng.gen_range(0, 340);
                self.gene[i].5 = rng.gen_range(0, 340);
                // radius value
                self.gene[i].6 = rng.gen_range(0, 50);
            }
        }
    }

    pub fn len(&self) -> usize{
        self.gene.len()
    }
}