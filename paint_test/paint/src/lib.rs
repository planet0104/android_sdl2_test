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
use rand::prelude::*;
use std::time::{Duration, Instant};
use std::sync::mpsc::channel;
use log::LevelFilter;
use picture::Picture;
use image::{ImageOutputFormat, RgbaImage};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;

const CROSSOVER_RATE: f32 = 0.7;
const MUTATION_RATE: f32 = 0.01;
const NUM_THREAD: usize = 8;//8个线程
const NUM_ELITE: usize = 4;//精英选择个数
const NUM_COPIES_ELITE: usize = 1; //每个精英复制数
const POP_SIZE: usize = 8*NUM_THREAD+NUM_ELITE*NUM_COPIES_ELITE;//人口数量

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
    fn new(buffer: RgbaImage) -> Populations{
        //初始化随机群体
        let mut pictures = vec![];
        for _ in 0..POP_SIZE{
            pictures.push(Picture::new());
        }
        Populations{pictures, total_fitness:0.0, generations:0, target: buffer }
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
            picture.calc_fitness(buffer);
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
        info!("代数:{} 最高分:{}", self.generations, self.pictures[0].fitness);
        //新群体
        let mut new_pop = vec![];
        //精英选择
        for i in 0..NUM_ELITE{
            for _ in 0..NUM_COPIES_ELITE{
                new_pop.push(self.pictures[i]);
            }
        }

        let (tx, rx) = channel();
        let elite_count = new_pop.len();
        let new_pop = Arc::new(Mutex::new(new_pop));
        for _thread in 0..NUM_THREAD{
            let buffer = self.target.clone();
            let child_count = (POP_SIZE-elite_count)/NUM_THREAD;
            let mut parents = vec![];
            for _ in 0..child_count/2{
                //每次生成两个孩子
                parents.push(*self.roulette_wheel_selection());
                parents.push(*self.roulette_wheel_selection());
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
                    let (mut baby1, mut baby2) = mum.crossover(CROSSOVER_RATE, &dad);
                    //变异
                    baby1.mutate(MUTATION_RATE);
                    baby1.mutate(MUTATION_RATE);
                    //计算适应分
                    baby1.calc_fitness(&buffer);
                    baby2.calc_fitness(&buffer);

                    childs.push(baby1);
                    childs.push(baby2);
                }
                let mut new_pop = new_pop_clone.lock().unwrap();
                new_pop.append(&mut childs);
                if new_pop.len() == POP_SIZE{
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
    let (width, height) = (buffer.width(), buffer.height());

    info!("像素:{}x{}", width, height);
    
    let now = Instant::now();
    let mut populations = Populations::new(buffer);

    populations.calc_fitness(&img.to_rgba());
    info!("Populations::calc_fitness 耗时:{}ms", duration_to_milis(&now.elapsed()));
    for _ in 0..3000{
        populations.epoch();
    }
    info!("最高分数:{} 耗时:{}ms", populations.pictures[0].fitness, duration_to_milis(&now.elapsed()));

    let mut buffer = vec![];
    let _ = populations.pictures[0].render(width, height).write_to(&mut buffer, ImageOutputFormat::BMP);
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