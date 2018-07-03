use imageproc::drawing::*;
use image::{Rgba, ImageOutputFormat, RgbaImage, DynamicImage};
use ::line::Line;
use imageproc::rect::Rect;
use rand;
use rand::Rng;
use math::Matrix2D;

pub struct Picture{
    width: u32,
    height: u32,
    image: Option<DynamicImage>,
    lines: Vec<Line>,
    fitness: f32,
}
impl Picture{
    pub fn new(width: u32, height: u32, line_count: u32) -> Picture{
        let mut lines = vec![];
        for _ in 0..line_count{
            let mut line = Line::new();
            Picture::random_line(width, height, &mut line);
            lines.push(line);
        }
        Picture{fitness:0.0,  lines, width, height, image:None}
    }

    //随机线条
    fn random_line(width:u32, height:u32, line:&mut Line){
        let mut rng = rand::thread_rng();
        //随机位置
        let (x, y) = (rng.gen_range(0, width), rng.gen_range(0, width));
        line.x = x as f32;
        line.y = y as f32;
        //随机大小
        line.scale = rng.gen_range(0, width/2) as f32;
        //随机角度 +=90度
        line.rotation = rng.gen_range(-3.14159265/2.0, 3.14159265/2.0);
    }

    pub fn render(&mut self){
        let mut img = DynamicImage::new_rgba8(self.width, self.height);
        draw_filled_rect_mut(&mut img, Rect::at(0, 0).of_size(self.width, self.height), Rgba([255, 255, 255, 255]));
        for line in &self.lines{
            //创建一个转换矩阵
            let mut matrix = Matrix2D::new_identity();
            //变比
            matrix.scale(line.scale, line.scale);
            //旋转
            matrix.rotate(line.rotation);
            //移动
            matrix.translate(line.x, line.y);
            let mut points = VE
            draw_line_segment_mut(
                &mut img,
                (line.x1 as f32, line.y1 as f32),
                (line.x2 as f32, line.y2 as f32),
                Rgba([0, 0, 0, 255]),
            );
        }
        self.image = Some(img);
    }

    //计算适应分, 图像为白色背景, 黑色前景。
    pub fn calc_fitness_score(&mut self, buffer:&RgbaImage) -> f32{
        //将线条绘制成图片
        self.render();
        //对比图片像素
        let test_buffer = self.image.as_ref().unwrap().to_rgba();
        let mut target_pixels = buffer.pixels();
        let mut test_pixels = test_buffer.pixels();
        self.fitness = 0.0;
        while let Some(pixel) = test_pixels.next(){
            let target_pixel = target_pixels.next().unwrap();
            if pixel[0] == target_pixel[0]{
                self.fitness += 1.0;
            }else{
                self.fitness -= 1.0;
            }
        }
        self.fitness
    }

    //变异
    pub fn mutate(&mut self){
        //[变长]、[变短]、[旋转]、[平移]、[替换]
        /*
            scale 变长,变短
            rotation 旋转
            x,y 平移/位置
            random 随机变化
            */
        
    }
}