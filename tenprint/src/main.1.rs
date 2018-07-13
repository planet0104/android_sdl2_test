extern crate brainfuck;
extern crate rand;
#[macro_use]
extern crate log;
extern crate env_logger;

mod context;

use brainfuck::parser;
use context::Context;
use std::time::{Duration, Instant};
use rand::random;
use log::LevelFilter;
use std::sync::mpsc::channel;
use std::thread;
use std::sync::{Arc, Mutex};

/*
f64数组 代表genome(基因组)
每个f64(8个字节 gene 基因)代表一条指令

AI程序的工作原理如下:

一个基因组由一个f64数组组成。
每个基因对应Brainf-ck编程语言中的指令。
从一群随机基因组开始。
将每个f64转换成相应的指令，编码成结果程序，并执行这个程序。
根据程序的控制台输出获取每个程序的适应分数，并对它们进行排名。
使用赌轮选择，杂交和变异将最佳基因组配对在一起，以产生新一代。
用新一代重复该过程，直到达到目标适应分数。

解释指令集

Brainf-ck由以下指令集组成：

1   >   递增指针。
2   <   减少指针。
3   +   递增指针处的字节。
3   -   减少指针处的字节。
5   .   输出指针处的字节。
6   ,   输入一个字节并将其存储在指针的字节中。
7   [   如果指针处的字节为零，则跳过匹配]。
8   ]   向后跳转到匹配[除非指针处的字节为零。

*/

const MUTATION_RATE: f64 = 0.05;
const CROSSOVER_RATE: f64 = 0.80;
const INITIAL_GENOME_SIZE: usize = 100;
const NUM_ELITE: usize = 4;//精英选择个数
const NUM_COPIES_ELITE: usize = 1; //每个精英复制数
const NUM_THREAD: usize = 2;//线程数
const POPULATION_SIZE: usize = 50*NUM_THREAD+NUM_ELITE*NUM_COPIES_ELITE;//人口数量

//基因组
#[derive(Copy)]
pub struct Genome {
    fitness: f64,
    genes: Vec<f64>,
}

impl Genome {
    fn new() -> Genome{
        Genome{
            fitness: 1.0,
            genes: vec![]
        }
    }

    fn length(&self) -> usize{
        self.genes.len()
    }

    fn random() -> Genome{
        Genome{
            fitness: 1.0,
            genes: vec![random(); INITIAL_GENOME_SIZE]
        }
    }

    /*
       通过插入，替换，删除，移位进行突变。
       - 在基因组中选择一个索引。
       - 如果插入，则在该位置插入一个变异位。 其余位向上移动一个索引。 最后一位被删除了。
       - 如果替换，则在该位置设置变异位。
       - 如果删除，所有位都在该位置向下移动。 在数组的末尾添加一个变异位。
       - 如果移位，所有位都从位置0开始向上或向下移动。如果向上，则最后一位向前移动。 如果向下，第一位就会结束。
     */
    fn mutate(&mut self){
        for pos in 0..self.genes.len(){
            if random::<f64>() < MUTATION_RATE{
                //选择变异类型
                let r = random::<f64>();
                if r <= 0.25 {
                    //插入突变
                    let mutation_index = pos;
                    //变异之前备份当前位
                    let mut shift_bit = self.genes[mutation_index];
                    //在变异位设置随机数
                    self.genes[mutation_index] = random();
                    
                    //将位向上或向下凹陷1。
                    let up = random::<f64>() >= 0.5;
                    if up{//插入并删除末尾
                        for i in mutation_index+1..self.length{
                            let next_shift_bit = self.genes[i];
                            self.genes[i] = shift_bit;
                            shift_bit = next_shift_bit;
                        }
                    }else{//插入并删除第一个
                        for i in (0..=mutation_index).rev(){
                            let next_shift_bit = self.genes[i];
                            self.genes[i] = shift_bit;
                            shift_bit = next_shift_bit;
                        }
                    }
                }else if r <= 0.5{
                    //删除突变
                    let mutation_index = pos;
                    let up = random::<f64>() >= 0.5;
                    if up{//删除并在开头插入
                        for i in (1..=mutation_index).rev(){
                            self.genes[i] = self.genes[i-1];
                        }
                        self.genes[0] = random();
                    }else{//删除并在末尾插入
                        for i in mutation_index..self.length-1{
                            self.genes[i] = self.genes[i+1]
                        }
                        self.genes[self.length-1] = random();
                    }
                }else if r <= 0.75{
                    //转移/旋转突变
                    let up = random::<f64>() >= 0.5;
                    if up{
                        // 1,2,3 => 3,1,2
                        let mut shift_bit = self.genes[0];
                        for i in 0..self.length{
                            if i>0{
                                let temp = self.genes[i];
                                self.genes[i] = shift_bit;
                                shift_bit = temp;
                            }else{
                                self.genes[i] = self.genes[self.length-1];
                            }
                        }
                    }else{
                        // 1,2,3 => 2,3,1
                        let mut shift_bit = self.genes[self.length-1];
                        for i in (0..=self.length-1).rev(){
                            if i<self.length-1{
                                let temp = self.genes[i];
                                self.genes[i] = shift_bit;
                                shift_bit = temp;
                            }else{
                                self.genes[i] = self.genes[0];
                            }
                        }
                    }
                }else{
                    //替换突变
                    self.genes[pos] = random();
                }
            }
        }
    }

    fn crossover(&self, genome:&Genome) -> (Genome, Genome){
        if random::<f64>()>CROSSOVER_RATE{
            return (self.clone(), genome.clone());   
        }
        let pos = (random::<f64>()*self.length as f64) as usize;
        let mut child1 = Genome::new();
        let mut child2 = Genome::new();
        for i in 0..self.length{
            if i<pos{
                child1.genes[i] = self.genes[i];
                child2.genes[i] = genome.genes[i];
            }else{
                child1.genes[i] = genome.genes[i];
                child2.genes[i] = self.genes[i];
            }
        }
        (child1, child2)
    }

    fn to_bf(&self) -> String {
        let mut bf = String::new();
        for gene in self.genes.iter() {
            let d = *gene;
            if d <= 0.125 {
                bf.push('>');
            } else if d <= 0.25 {
                bf.push('<');
            } else if d <= 0.375 {
                bf.push('+');
            } else if d <= 0.5 {
                bf.push('-');
            } else if d <= 0.625 {
                bf.push('.');
            } else if d <= 0.75 {
                //bf.push(',');
                bf.push('.');
            } else if d <= 0.875 {
                bf.push('[');
            } else {
                bf.push(']');
            }
        }
        bf
    }

    fn run(&self) -> String{
        let mut context = Context::new();
        let program = self.to_bf().replace("[]", "");
        if let Ok(block) = parser::parse(program.as_bytes()) {
            context.run(&block);
        }
        context.out
    }

    fn calc_fitness(&mut self, target: &str){
        let target = target.as_bytes();
        self.fitness = 0.0;
        let out = self.run();
        let out_bytes = out.as_bytes();
        for i in 0..target.len() {
            if out_bytes.len()>i{
                self.fitness += 255.0 - (out_bytes[i] as f64 - target[i] as f64).abs();
            }
        }
    }
}

impl Clone for Genome {
    fn clone(&self) -> Genome {
        Genome{
            fitness: self.fitness,
            genes: self.genes,
            length: self.length
        }
    }
}

pub struct GA {
    target: String,
    populations: Vec<Genome>,
    total_fitness: f64,
    generations: usize,
}

impl GA {
    fn new(target: &str) -> GA {
        let mut populations = vec![];
        for _ in 0..POPULATION_SIZE{
            populations.push(Genome::random());
        }
        GA {
            target: String::from(target),
            generations: 0,
            total_fitness: 0.0,
            populations
        }
    }

    fn roulette_selection(&self) -> usize{
        //生成0和总体适应分之间的随机数
        let slice = random::<f64>() * self.total_fitness;
        let mut fitness_total = 0.0;
        let mut selected_pos = 0;
        for i in 0..self.populations.len(){
            fitness_total += self.populations[i].fitness;
            //如果当前适应分>随机数，返回此处的染色体
            if fitness_total > slice{
                selected_pos = i;
                break;
            }
        }
        selected_pos
    }

    //下一代
    fn epoch(&mut self){
        //计算总适应分
        self.total_fitness = 0.0;
        for p in &mut self.populations{
            self.total_fitness += p.fitness;
        }

        //按照得分排序
        self.populations.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());
        let out = self.populations[0].run();
        println!("program={} out={:?}", self.populations[0].to_bf(), out.get(0..5));
        info!("人口:{} 代数:{} 最高分:{}", self.populations.len(), self.generations, self.populations[0].fitness);
        //新群体
        let mut new_pop = vec![];
        //精英选择
        for i in 0..NUM_ELITE{
            for _ in 0..NUM_COPIES_ELITE{
                new_pop.push(self.populations[i]);
            }
        }
        
        let (tx, rx) = channel();
        let elite_count = new_pop.len();
        let new_pop = Arc::new(Mutex::new(new_pop));
        for tid in 0..NUM_THREAD{
            let target = self.target.clone();
            let child_count = (POPULATION_SIZE-elite_count)/NUM_THREAD;
            let mut parents = vec![];
            for _ in 0..child_count/2{
                //每次生成两个孩子
                parents.push(self.populations[self.roulette_selection()]);
                parents.push(self.populations[self.roulette_selection()]);
            }
            let tx = tx.clone();
            let new_pop_clone = new_pop.clone();
            thread::spawn(move || {
                let mut childs = vec![];
                //println!("{}.start", tid);
                //每次生成两个孩子
                while childs.len()<child_count as usize{
                    let mum = parents.pop().unwrap();
                    let dad = parents.pop().unwrap();
                    //杂交
                    let (mut baby1, mut baby2) = mum.crossover(&dad);
                    //变异
                    baby1.mutate();
                    baby1.mutate();
                    //计算适应分
                    baby1.calc_fitness(&target);
                    baby2.calc_fitness(&target);

                    childs.push(baby1);
                    childs.push(baby2);
                }
                //println!("{}.end", tid);
                let mut new_pop = new_pop_clone.lock().unwrap();
                new_pop.append(&mut childs);
                //println!("{}.new_pop.len()={}", tid, new_pop.len());
                if new_pop.len() == POPULATION_SIZE{
                    let mut pops = vec![];
                    pops.append(&mut new_pop);
                    tx.send(pops).unwrap();
                }
            });
        }

        //替换新的群体
        self.populations.clear();
        self.populations.append(&mut rx.recv().unwrap());
        self.generations += 1;
    }
}

fn main() {
    env_logger::Builder::from_default_env()
    //.default_format_timestamp(false)
    .filter_level(LevelFilter::Info)
    .init();
    //let hello_world = include_bytes!("../hello_world.bf");

    //let program = b"++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";

    let mut ga = GA::new("Hello");
    for _ in 0..50000{
        ga.epoch();
    }
    // for i in 0..ga.populations.len(){
    //     ga.populations[i].calc_fitness("Hello World!");
    // }

    // let dad = ga.populations[ga.roulette_selection()];
    // let mum = ga.populations[ga.roulette_selection()];
    // let (mut child1, mut child2) = dad.crossover(&mum);
    // child1.calc_fitness("Hello World!");
    // child2.calc_fitness("Hello World!");

    println!("end.");

    //let now = Instant::now();
    // let fitness = ga.calc_fitness(program, "Hello World!");
    // println!("耗时:{}ms", duration_to_milis(&now.elapsed()));
    // println!("fitness:{}", fitness);
}

pub fn duration_to_milis(duration: &Duration) -> f64 {
    duration.as_secs() as f64 * 1000.0 + duration.subsec_nanos() as f64 / 1_000_000.0
}
