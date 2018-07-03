use imageproc::drawing::*;
use image::{Rgba, RgbaImage, DynamicImage};
use ::line::Line;
use imageproc::rect::Rect;

pub struct Picture{
    pub width: u32,
    pub height: u32,
    pub image: DynamicImage,
    pub lines: Vec<Line>,
    pub fitness: f32,
}
impl Picture{
    pub fn new(width: u32, height: u32, line_count: u32) -> Picture{
        let mut lines = vec![];
        for _ in 0..line_count{
            let mut line = Line::new();
            line.randomize(width, height);
            lines.push(line);
        }
        Picture{fitness:0.0,  lines, width, height, image:DynamicImage::new_rgba8(width, height)}
    }

    pub fn from_picture(pic: &Picture) ->Picture{
        Picture{
            fitness: pic.fitness,
            width: pic.width,
            height: pic.height,
            lines: pic.lines.clone(),
            image: pic.image.clone(),
        }
    }

    pub fn from_lines(width: u32, height: u32, lines: Vec<Line>) ->Picture{
        Picture{
            fitness: 0.0,
            width: width,
            height: height,
            lines: lines,
            image: DynamicImage::new_rgba8(width, height),
        }
    }

    //线条坐标转换
    fn world_transform(&mut self){
        for line in &mut self.lines{
            line.world_transform();
        }
    }

    //绘制
    pub fn render(&mut self){
        self.world_transform();
        //白色背景
        draw_filled_rect_mut(&mut self.image, Rect::at(0, 0).of_size(self.width, self.height), Rgba([255, 255, 255, 255]));
        for line in &self.lines{
            //黑色线条
            let verts = line.verts_trans();
            draw_line_segment_mut(
                &mut self.image,
                (verts[0].0, verts[0].1),
                (verts[1].0, verts[1].1),
                Rgba([0, 0, 0, 255]),
            );
        }
    }

    //计算适应分, 图像为白色背景, 黑色前景。
    pub fn calc_fitness_score(&mut self, buffer:&RgbaImage) -> f32{
        //将线条绘制成图片
        self.render();
        //对比图片像素
        let test_buffer = self.image.to_rgba();
        let mut target_pixels = buffer.pixels();
        let mut test_pixels = test_buffer.pixels();
        self.fitness = 0.0;
        while let Some(pixel) = test_pixels.next(){
            let target_pixel = target_pixels.next().unwrap();
            if pixel[0] == target_pixel[0]{
                self.fitness += 1.0;
            }
        }
        self.fitness
    }
}