//#[cfg(target_os="android")]
//extern crate android_log_sys;
extern crate rand;

use rand::prelude::*;
use std::time::{Duration, Instant};

extern crate image;
extern crate imageproc;
mod math;
use image::{Rgba, ImageOutputFormat, RgbaImage, DynamicImage};
use imageproc::drawing::*;
use imageproc::rect::Rect;
use std::path::Path;
use math::Matrix2D;

const CROSSOVER_RATE: f32 = 0.7;
const MUTATION_RATE: f32 = 0.2;

pub fn duration_to_milis(duration: &Duration) -> f64 {
    duration.as_secs() as f64 * 1000.0 + duration.subsec_nanos() as f64 / 1_000_000.0
}

#[derive(Debug, Copy, Clone)]
struct Line{
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32
}
impl Line{
    fn new(x1:u32, y1:u32, x2:u32, y2:u32) -> Line{ Line{x1, y1, x2, y2}  }
}

struct Picture{
    width: u32,
    height: u32,
    image: Option<DynamicImage>,
    lines: Vec<Line>,
    fitness: f32,
}
impl Picture{
    fn new(width: u32, height: u32, line_count: u32) -> Picture{
        let mut rng = rand::thread_rng();
        let mut lines = vec![];
        for _ in 0..line_count{
            let (x1, x2) = (rng.gen_range(0, width), rng.gen_range(0, width));
            let (y1, y2) = (rng.gen_range(0, height), rng.gen_range(0, height));
            lines.push(Line::new(x1, y1, x2, y2));
        }
        Picture{fitness:0.0,  lines, width, height, image:None}
    }

    fn render(&mut self){
        let mut img = DynamicImage::new_rgba8(self.width, self.height);
        draw_filled_rect_mut(&mut img, Rect::at(0, 0).of_size(self.width, self.height), Rgba([255, 255, 255, 255]));
        for line in &self.lines{
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
    fn calc_fitness_score(&mut self, buffer:&RgbaImage) -> f32{
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
    fn mutate(&mut self){
        //[变长]、[变短]、[旋转]、[平移]、[替换]
        
    }
}

struct Populations{
    pictures: Vec<Picture>,
    total_fitness: f32,
}

impl Populations{
    fn new(size: u32, width:u32, height: u32, lines: u32) -> Populations{
        //初始化随机群体
        let mut pictures = vec![];
        for _ in 0..size{
            pictures.push(Picture::new(width, height, lines));
        }
        Populations{pictures, total_fitness:0.0 }
    }

    //杂交
    fn crossover(&self, mum:&Picture, dad:&Picture) -> (Picture, Picture) {
        let mut rng = thread_rng();
        let c:f32 = rng.gen();
        if c>CROSSOVER_RATE {
            (Picture{
                fitness: 0.0,
                width: mum.width,
                height: mum.height,
                lines: mum.lines.clone(),
                image: None,
            }, Picture{
                fitness: 0.0,
                width: dad.width,
                height: dad.height,
                lines: dad.lines.clone(),
                image: None,
            })
        }else{
            //确定交叉点
            let index1 = rng.gen_range(0, mum.lines.len());
            let index2 = rng.gen_range(index1, mum.lines.len());
            let mut baby1 = vec![];
            let mut baby2 = vec![];

            for i in 0..mum.lines.len(){
                if i>=index1 && i<=index2{
                    //交换
                    baby1.push(dad.lines[i]);
                    baby2.push(mum.lines[i]);
                }else{
                    //如果在交叉点以外，保留相同的基因
                    baby1.push(mum.lines[i]);
                    baby2.push(dad.lines[i]);
                }
            }
            (Picture{
                fitness: 0.0,
                width: mum.width,
                height: mum.height,
                lines: baby1,
                image: None,
            }, Picture{
                fitness: 0.0,
                width: dad.width,
                height: dad.height,
                lines: baby2,
                image: None,
            })
        }
    }

    //赌轮选择
    fn roulette_wheel_selection(&self) -> &Picture{
        //生成0和总体适应分之间的随机数
        let mut rng = rand::thread_rng();
        let slice = rng.gen_range(0.0, self.total_fitness);
        let mut fitness_total = 0.0;
        let mut selected_picture = 0;
        for i in 0..self.pictures.len(){
            fitness_total += self.pictures[i].fitness;
            //如果当前适应分>随机数，返回此处的染色体
            if fitness_total > slice{
                selected_picture = i;
                break;
            }
        }
        &self.pictures[selected_picture]
    }

    fn epoch(&mut self, buffer:&RgbaImage){
        //计算个体的适应分
        self.total_fitness = 0.0;
        for picture in &mut self.pictures{
            picture.calc_fitness_score(buffer);
            self.total_fitness += picture.fitness;
        }
        //按照得分排序
        self.pictures.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());
        //精英选择

        //赌轮选择
        let mut new_pop = vec![];
        while new_pop.len() < self.pictures.len(){
            //选择父母
            let mum = self.roulette_wheel_selection();
            let dad = self.roulette_wheel_selection();
            //杂交
            let (baby1, baby2) = self.crossover(mum, dad);
            //变异

            new_pop.push(baby1);
            new_pop.push(baby2);
        }
    }
}

#[no_mangle]
pub fn process(path: &str) -> Vec<u8> {
    let img = image::open(&Path::new(path)).unwrap();
    let buffer = img.to_rgba();

    /*
    线条: x1,y1,x2,y2
    图片: 由300个线条组成
    群体: 100个图片组成

    适应分: 逐个检查像素, 匹配+1, 不匹配-1
    选择: 赌轮选择/精英选择
    杂交: 置换杂交/两点杂交
    变异: 对个体中的某些线条进行 [变长]、[变短]、[旋转]、[位移]、[或替换为新的随机线条]

    */
    println!("像素:{}x{} 满分:{}", buffer.width(), buffer.height(), buffer.width()*buffer.height());

    let group_size = 100;
    let line_count = 300;
    let mut populations = Populations::new(group_size, buffer.width(), buffer.height(), line_count);

    let now = Instant::now();
    
    populations.epoch(&buffer);

    println!("耗时:{}ms", duration_to_milis(&now.elapsed()));

    for picture in &mut populations.pictures{
        println!("score={}", picture.fitness);
    }

    let mut img = DynamicImage::new_rgba8(200, 200);
    draw_filled_rect_mut(&mut img, Rect::at(0, 0).of_size(200, 200), Rgba([255, 255, 255, 255]));
    let mut points = vec![(50.0, 50.0), (50.0, 100.0)];
    draw_line_segment_mut(
        &mut img,
        (points[0].0, points[0].1),
        (points[1].0, points[1].1),
        Rgba([0, 0, 0, 255]),
    );

    //创建一个转换矩阵
    let mut matrix = Matrix2D::new_identity();
    //变比
    matrix.scale(1.0, 2.0);
    //旋转
    //matrix.rotate(10.0);
    //转换
    //matrix.translate(60.0, 60.0);

    println!("{:?}", points);
    matrix.transform(&mut points);
    println!("{:?}", points);

    draw_line_segment_mut(
        &mut img,
        (points[0].0, points[0].1),
        (points[1].0, points[1].1),
        Rgba([255, 0, 0, 255]),
    );

    let mut buffer = vec![];
    let _ = img.write_to(&mut buffer, ImageOutputFormat::BMP);
    buffer
}

//定义Android JNI接口
// #[cfg(target_os="android")]
// #[allow(non_snake_case)]
// pub mod android {
//     extern crate jni;
//     use android_log_sys::{__android_log_print, LogPriority};

//     use super::*;
//     use self::jni::JNIEnv;
//     use self::jni::objects::{JClass, JString};
//     use self::jni::sys::{jstring};
//     use std::ffi::CString;

//     fn log(tag:&str, text:&str){
//         // let tag = CString::new(tag).unwrap();
//         // let text = CString::new(text).unwrap();
//         // unsafe{ __android_log_print(LogPriority::DEBUG as i32, tag.as_ptr(), text.as_ptr()); }
//     }

//     #[no_mangle]
//     pub unsafe extern "C" fn Java_jna_test_com_testjna_Lib_start(env: JNIEnv, _: JClass, from: JString) -> jstring {
//         log("libpaint", "start.");
//         let name:String = env.get_string(from).expect("无法取到名字!").into();
//         let echo = format!("{}HelloWorld!", name);
//         let output = env.new_string(echo).expect("java字符串创建失败!");
//         output.into_inner()
//     }
// }