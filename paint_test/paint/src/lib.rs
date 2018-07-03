//#[cfg(target_os="android")]
//extern crate android_log_sys;
extern crate rand;
#[macro_use]
extern crate log;
extern crate env_logger;
//extern crate num_cpus;
extern crate image;
extern crate imageproc;
mod math;
mod line;
mod picture;
mod dna;
use rand::prelude::*;
use std::time::{Duration, Instant};
use std::sync::mpsc::channel;
use log::LevelFilter;
use picture::Picture;
use image::{ImageOutputFormat, RgbaImage, DynamicImage};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;

const CROSSOVER_RATE: f32 = 0.99;
const MUTATION_RATE: f32 = 0.2;
const GROUP_SIZE: u32 = 24*8+16;// 8个线程

pub fn duration_to_milis(duration: &Duration) -> f64 {
    duration.as_secs() as f64 * 1000.0 + duration.subsec_nanos() as f64 / 1_000_000.0
}

struct Populations{
    pictures: Vec<Picture>,
    total_fitness: f32,
    generations: u32,
    target: RgbaImage,
}

impl Populations{
    fn new(size: u32, buffer: RgbaImage, lines: u32) -> Populations{
        //初始化随机群体
        let mut pictures = vec![];
        for _ in 0..size{
            pictures.push(Picture::new(buffer.width(), buffer.height(), lines));
        }
        Populations{pictures, total_fitness:0.0, generations:0, target: buffer }
    }

    //杂交
    fn crossover(mum:&Picture, dad:&Picture) -> (Picture, Picture) {
        let mut rng = thread_rng();
        let c:f32 = rng.gen();
        if c>CROSSOVER_RATE {
            (Picture::from_picture(mum), Picture::from_picture(dad))
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
            (Picture::from_lines(mum.width, mum.height, baby1),
            Picture::from_lines(dad.width, dad.height, baby2))
        }
    }

    fn mutate(pic:&mut Picture){
        for line in &mut pic.lines{
            //突变每一个线条
            if rand::random::<f32>() < MUTATION_RATE{
                line.mutate(pic.width, pic.height);   
            }
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

    //计算适应分
    fn calc_fitness(&mut self, buffer:&RgbaImage){
        for picture in &mut self.pictures{
            picture.calc_fitness_score(buffer);
        }
    }

    fn epoch(&mut self){
        //计算总适应分
        self.total_fitness = 0.0;
        for picture in &mut self.pictures{
            self.total_fitness += picture.fitness;
        }

        //按照得分排序
        self.pictures.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());
        info!("代数:{} 最高分:{} 匹配度:{}%", self.generations, self.pictures[0].fitness, (self.pictures[0].fitness as f32/(self.target.width()*self.target.height()) as f32)*100.0);
        //新群体
        let mut new_pop = vec![];
        //16个精英
        new_pop.push(Picture::from_picture(&self.pictures[0]));
        new_pop.push(Picture::from_picture(&self.pictures[0]));
        new_pop.push(Picture::from_picture(&self.pictures[0]));
        new_pop.push(Picture::from_picture(&self.pictures[0]));
        new_pop.push(Picture::from_picture(&self.pictures[1]));
        new_pop.push(Picture::from_picture(&self.pictures[1]));
        new_pop.push(Picture::from_picture(&self.pictures[1]));
        new_pop.push(Picture::from_picture(&self.pictures[1]));
        new_pop.push(Picture::from_picture(&self.pictures[2]));
        new_pop.push(Picture::from_picture(&self.pictures[2]));
        new_pop.push(Picture::from_picture(&self.pictures[2]));
        new_pop.push(Picture::from_picture(&self.pictures[2]));
        new_pop.push(Picture::from_picture(&self.pictures[3]));
        new_pop.push(Picture::from_picture(&self.pictures[3]));
        new_pop.push(Picture::from_picture(&self.pictures[3]));
        new_pop.push(Picture::from_picture(&self.pictures[3]));

        //let cpu_cores = num_cpus::get();
        let (tx, rx) = channel();
        let elite_count = new_pop.len();
        let new_pop = Arc::new(Mutex::new(new_pop));
        let thread_count = 8;
        for _thread in 0..thread_count{
            let buffer = self.target.clone();
            //32*6+16=208
            let child_count = (GROUP_SIZE-elite_count as u32)/thread_count;
            let mut parents = vec![];
            for _ in 0..child_count/2{
                //每次生成两个孩子
                parents.push(Picture::from_picture(self.roulette_wheel_selection()));
                parents.push(Picture::from_picture(self.roulette_wheel_selection()));
            }
            let tx = tx.clone();
            let new_pop_clone = new_pop.clone();
            thread::spawn(move || {
                let mut childs = vec![];
                //每次生成两个孩子
                while childs.len()<child_count as usize{
                    let mum = parents.pop().unwrap();
                    let dad = parents.pop().unwrap();
                    //杂交
                    let (mut baby1, mut baby2) = Populations::crossover(&mum, &dad);
                    //变异
                    Populations::mutate(&mut baby1);
                    Populations::mutate(&mut baby2);
                    //计算适应分
                    baby1.calc_fitness_score(&buffer);
                    baby2.calc_fitness_score(&buffer);

                    childs.push(baby1);
                    childs.push(baby2);
                }
                let mut new_pop = new_pop_clone.lock().unwrap();
                new_pop.append(&mut childs);
                if new_pop.len() == GROUP_SIZE as usize{
                    let mut pops = vec![];
                    pops.append(&mut new_pop);
                    tx.send(pops).unwrap();
                }
            });
        }

        //替换新的群体
        self.pictures.clear();
        self.pictures.append(&mut rx.recv().unwrap());
        self.generations += 1;
    }
}

#[no_mangle]
pub fn process(path: &str) -> Vec<u8> {
    env_logger::Builder::from_default_env()
        //.default_format_timestamp(false)
        .filter_level(LevelFilter::Info)
        .init();

    let img = image::open(&Path::new(path)).unwrap();
    let buffer = img.to_rgba();

    info!("像素:{}x{} 满分:{}", buffer.width(), buffer.height(), buffer.width()*buffer.height());
    
    let line_count = 450;
    let mut populations = Populations::new(GROUP_SIZE, buffer, line_count);

    let now = Instant::now();
    populations.calc_fitness(&img.to_rgba());
    for _ in 0..1000{
        populations.epoch();
    }
    info!("最高分数:{} 耗时:{}ms", populations.pictures[0].fitness, duration_to_milis(&now.elapsed()));

    let mut buffer = vec![];
    let _ = populations.pictures[0].image.write_to(&mut buffer, ImageOutputFormat::BMP);
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