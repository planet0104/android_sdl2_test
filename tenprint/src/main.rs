extern crate rand;

mod pbrain;
use std::time::{Duration, Instant};
use rand::random;
use std::sync::mpsc::channel;
use std::thread;
use std::sync::{Arc, Mutex};
use std::str;
use rand::{thread_rng, Rng};
use pbrain::PBrain;

/*
char数组 代表genome(基因组)
每个char代表一条指令

AI程序的工作原理如下:

一个基因组由一个char数组组成。
每个基因对应pbrain编程语言中的指令。
从一群随机基因组开始。
将每个char转换成相应的指令，编码成结果程序，并执行这个程序。
根据程序的控制台输出获取每个程序的适应分数，并对它们进行排名。
使用赌轮选择，杂交和变异将最佳基因组配对在一起，以产生新一代。
用新一代重复该过程，直到达到目标适应分数。
*/

const INSTRUCTION_SET:[char; 11] = ['>', '<', '+', '-', '.', ',', '[', ']', '(', ')', ':'];

const MUTATION_RATE: f64 = 0.09;//0.05~0.3
const CROSSOVER_RATE: f64 = 0.6;//0.7
const INITIAL_GENOME_SIZE: usize = 100;
const NUM_ELITE: usize = 3;//精英选择个数
const NUM_COPIES_ELITE: usize = 3; //每个精英复制数
const NUM_THREAD: usize = 4;//线程数
const POPULATION_SIZE: usize = 30*NUM_THREAD+NUM_ELITE*NUM_COPIES_ELITE;//人口数量

const MAX_ITERATION_COUNT:u64 = 1000;

//基因组
pub struct Genome {
    fitness: f64,
    genes: Vec<char>,
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
        let mut rng = thread_rng();
        let mut genes = vec![];
        for _ in 0..INITIAL_GENOME_SIZE{
            genes.push(*rng.choose(&INSTRUCTION_SET).unwrap());
        }
        Genome{
            fitness: 1.0,
            genes: genes
        }
    }

    //变异
    pub fn mutate(&mut self, mutation_rate: f64) {
        let len = self.length();
        let mut rng = thread_rng();
        //每个基因进行变异
        for i in 0..len {
            if rng.gen::<f64>() < mutation_rate {
                if rng.gen::<f64>() < 0.5 {
                    //突变     
                    self.genes[i] = *rng.choose(&INSTRUCTION_SET).unwrap();
                }else{
                    //旋转突变
                    if rng.gen::<f64>() >= 0.5{
                        self.genes.rotate_right(1);
                    }else{
                        self.genes.rotate_left(1);
                    }
                }
            }
        }

        //随机替换一段
        // if rng.gen::<f64>() < mutation_rate {
        //     let p1 = rng.gen_range(0, len);
        //     let p2 = rng.gen_range(p1, len);
        //     if p2-p1>0{
        //         //生成一个或大或小的新基因片段
        //         self.genes.splice(
        //             p1..p2,
        //             (0..rng.gen_range(0, (p2 - p1) * 2)).map(|_| rng.gen()),
        //         );
        //     }
        // }
    }

    //杂交
    pub fn crossover(&self, genome: &Genome, crossover_rate: f64) -> (Genome, Genome) {
        let mut rng = thread_rng();

        if random::<f64>()>crossover_rate{
            return (self.clone(), genome.clone());   
        }

        let mut child1 = Genome::new();
        let mut child2 = Genome::new();

        let tp = random::<f64>() >= 0.5;

        if tp{
            //单点杂交: 从父母各取一个点，分别交换
            let p1 = rng.gen_range(0, self.length());
            let p2 = rng.gen_range(0, genome.length());
            
            child1.genes.extend_from_slice(&self.genes[0..p1]);
            child1.genes.extend_from_slice(&genome.genes[p2..genome.length()]);

            child2.genes.extend_from_slice(&genome.genes[0..p2]);
            child2.genes.extend_from_slice(&self.genes[p1..self.length()]);
        }else{
            //两点杂交: 从父母各取两个点，交换中间的部分
            let mut p11 = (random::<f64>()*self.length() as f64) as usize;
            let mut p12 = (random::<f64>()*self.length() as f64) as usize;
            if p11>p12{
                let tmp = p11;
                p11 = p12;
                p12 = tmp;
            }
            let mut p21 = (random::<f64>()*genome.length() as f64) as usize;
            let mut p22 = (random::<f64>()*genome.length() as f64) as usize;
            if p21>p22{
                let tmp = p21;
                p21 = p22;
                p22 = tmp;
            }
            //前段
            child1.genes.extend_from_slice(&self.genes[0..p11]);
            //中间段
            child1.genes.extend_from_slice(&genome.genes[p21..p22]);
            //尾段
            child1.genes.extend_from_slice(&self.genes[p12..self.length()]);

            //child2同上
            child2.genes.extend_from_slice(&genome.genes[0..p21]);
            child2.genes.extend_from_slice(&self.genes[p11..p12]);
            child2.genes.extend_from_slice(&genome.genes[p22..genome.length()]);
        }

        (child1, child2)
    }

    fn to_program(&self) -> String {
        let mut program = String::new();
        for c in &self.genes{
            program.push(*c);
        }
        program
    }

    fn run(&self) -> (String, u64){
        let mut result = (String::new(), MAX_ITERATION_COUNT);
        let program = self.to_program();

        let mut pbrain = PBrain::new(vec![], MAX_ITERATION_COUNT);
        //println!("{}", program);
        if let Ok(()) = pbrain.parse(program.chars()){
            result = (pbrain.output().clone(), pbrain.iteration_count());
        }

        result    
    }

    //输出字符串计算适应分函数
    fn calc_fitness_print_string(out:&str, target:&str) -> f64{
        let mut fitness = 0.0;
        let out_bytes = out.as_bytes();
        let target = target.as_bytes();
        //匹配分
        for i in 0..target.len() {
            if out_bytes.len()>i{
                fitness += 255.0 - (out_bytes[i] as f64 - target[i] as f64).abs();
            }
            //输出短于target, 减分
        }
        if out_bytes.len()>target.len(){
            //超出target的, 减分
            for _ in target.len()..out_bytes.len(){
                fitness -= 5.0;
            }
        }
        fitness
    }

    fn calc_fitness(&mut self, target: &str){
        self.fitness = 0.0;
        let program = self.to_program();
        let mut pbrain = PBrain::new(vec![], MAX_ITERATION_COUNT);
        if let Ok(()) = pbrain.parse(program.chars()){
            //输出字符串计算适应分
            self.fitness += Genome::calc_fitness_print_string(pbrain.output(), target);
        }else{
            self.fitness -= 9999.0;
        }

        //执行时间越短， 适应分越高
        self.fitness -= pbrain.iteration_count() as f64 * 5.0;

        //指令越多适应分越低
        self.fitness -= program.len() as f64/10.0;
    }

    pub fn from_genes(genes: Vec<char>) -> Genome {
        Genome {
            fitness: 0.0,
            genes: genes,
        }
    }
}

impl Clone for Genome {
    fn clone(&self) -> Genome {
        Genome{
            fitness: self.fitness,
            genes: self.genes.clone(),
        }
    }
}

pub struct GA {
    start_time: Instant,
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
            start_time: Instant::now(),
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

        let elapsed_ms = duration_to_milis(&self.start_time.elapsed());
        //println!("elapsed_ms={}", elapsed_ms);
        if elapsed_ms>=1000.0{
            self.start_time = Instant::now();
            let out = self.populations[0].run();
            println!("人口:{} 代数:{} 平均分:{} 最高分:{} out={:?} bf={}", self.populations.len(), self.generations, self.total_fitness/POPULATION_SIZE as f64, self.populations[0].fitness, out, self.populations[0].to_program());
        }
        //新群体
        let mut new_pop = vec![];
        //精英选择
        for i in 0..NUM_ELITE{
            for _ in 0..NUM_COPIES_ELITE{
                new_pop.push(self.populations[i].clone());
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
                parents.push(self.populations[self.roulette_selection()].clone());
                parents.push(self.populations[self.roulette_selection()].clone());
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
                    let (mut baby1, mut baby2) = mum.crossover(&dad, CROSSOVER_RATE);
                    //变异
                    baby1.mutate(MUTATION_RATE);
                    baby1.mutate(MUTATION_RATE);
                    //计算适应分
                    baby1.calc_fitness(&target);
                    baby2.calc_fitness(&target);

                    childs.push(baby1);
                    childs.push(baby2);
                }
                //println!("线程{}执行完毕 childs.len()={}.", tid, childs.len());
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
        //self.populations.append(&mut new_pop);
        self.generations += 1;
    }
}

fn main() {
    //reddit
    //>>--<-----+-----------------------------------------------+----------------------------------------------------------------------------------<--->-+----------.-------------.-..-++++++.+++++++++++.>>++<>
    //hello world
    let target = "Hi!";
    //let target_fitness = target.len() as f64*255.0;
    let mut ga = GA::new(target);
    let now = Instant::now();
    while {
        let (out, tick) = ga.populations[0].run();
        if out == target{
            let elapsed_ms = duration_to_milis(&now.elapsed());
            println!("generations={} 最终程序:{} 适应分:{} 结果:{} 耗时:{}ms tick={}", ga.generations, ga.populations[0].to_program(), ga.populations[0].fitness, out, elapsed_ms, tick);
            false
        }else{
            true
        }
    }{
        ga.epoch();
    }
    // for p in &ga.populations{
    //     println!("{}", p.to_bf());
    // }
}

pub fn duration_to_milis(duration: &Duration) -> f64 {
    duration.as_secs() as f64 * 1000.0 + duration.subsec_nanos() as f64 / 1_000_000.0
}
