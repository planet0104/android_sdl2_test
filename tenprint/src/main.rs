extern crate brainfuck;
extern crate rand;

mod context;
use std::panic;
use brainfuck::parser;
use context::Context;
use std::time::{Duration, Instant};
use rand::random;
use std::sync::mpsc::channel;
use std::thread;
use std::sync::{Arc, Mutex};
use std::str;
use rand::{thread_rng, Rng};
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

//const INSTRUCTION_SET:[u8] = ['>', '<', '+', '-', '.', ',', '[', ']'];
const INSTRUCTION_SET:[char; 7] = ['>', '<', '+', '-', '.', '[', ']'];

const MUTATION_RATE: f64 = 0.02;
const CROSSOVER_RATE: f64 = 0.3;
const INITIAL_GENOME_SIZE: usize = 100;
const NUM_ELITE: usize = 3;//精英选择个数
const NUM_COPIES_ELITE: usize = 3; //每个精英复制数
const NUM_THREAD: usize = 6;//线程数
const POPULATION_SIZE: usize = 30*NUM_THREAD+NUM_ELITE*NUM_COPIES_ELITE;//人口数量

const MAX_TICK:u64 = 10000;
const MAX_LOOP:u64 = 5000;

//基因组
pub struct Genome {
    out: String,
    fitness: f64,
    genes: Vec<char>,
}

impl Genome {
    fn new() -> Genome{
        Genome{
            out: String::new(),
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
            out: String::new(),
            fitness: 1.0,
            genes: genes
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

        let mut rng = thread_rng();

        let mut index = 0;

        while index<self.length(){
            if rng.gen::<f64>() < MUTATION_RATE{
                //选择变异类型
                let r = rng.gen::<f64>();
                if r <= 0.25 {
                    //插入突变
                    self.genes.insert(index, *rng.choose(&INSTRUCTION_SET).unwrap());
                    //删除开头或末尾的基因
                    if self.length()>1{
                        let up = rng.gen::<f64>() >= 0.5;
                        if up{//删除末尾
                            self.genes.pop();
                        }else{//删除第一个
                            self.genes.remove(0);
                        }
                    }
                    //跳过插入的基因
                    index += 1;
                }else if r <= 0.5{
                    //删除突变
                    self.genes.remove(index);
                    let up = rng.gen::<f64>() >= 0.5;
                    let ch = *rng.choose(&INSTRUCTION_SET).unwrap();
                    if up{//在开头插入
                        self.genes.insert(0, ch);
                    }else{//在末尾插入
                        self.genes.push(ch);
                    }
                }else if r <= 0.75{
                    //转移/旋转突变
                    let up = random::<f64>() >= 0.5;
                    if up{
                        if let Some(last) = self.genes.pop(){
                            self.genes.insert(0, last);
                        }
                    }else{
                        // 1,2,3 => 2,3,1
                        let first = self.genes.remove(0);
                        self.genes.push(first);
                    }
                }else{
                    //替换突变
                    self.genes[index] = *rng.choose(&INSTRUCTION_SET).unwrap();
                }
            }

            //下一个基因
            index += 1;
        }

        //- 循环每个f64突变
        //- 插入突变(随机插入一条指令)
        //- 删除突变(随机删除一条指令)
        //- 循环移位突变

        // //替换
        // for pos in 0..self.genes.len(){
        //     if rng.gen::<f64>() < MUTATION_RATE{
        //         self.genes[pos] = *rng.choose(&INSTRUCTION_SET).unwrap();
        //     }
        // }

        // //插入
        // if rng.gen::<f64>() < MUTATION_RATE{
        //     let insert_pos =  
        //     if self.length()==0{
        //         0
        //     }else{
        //         rng.gen_range(0, self.length())
        //     };
            
        //     self.genes.insert(insert_pos, *rng.choose(&INSTRUCTION_SET).unwrap());
        // }

        // //删除
        // if rng.gen::<f64>() < MUTATION_RATE && self.length()>0{
        //     let delete_pos = rng.gen_range(0, self.length());
        //     self.genes.remove(delete_pos);
        // }

        // //循环移动
        // if rng.gen::<f64>() < MUTATION_RATE && self.length()>0{
        //     //转移/旋转突变
        //     let up = rng.gen::<f64>() >= 0.5;
        //     if up{
        //         // 1,2,3 => 3,1,2
        //         if let Some(last) = self.genes.pop(){
        //             self.genes.insert(0, last);
        //         }
        //     }else{
        //         // 1,2,3 => 2,3,1
        //         let first = self.genes.remove(0);
        //         self.genes.push(first);
        //     }
        // }
    }

    fn crossover(&self, genome:&Genome) -> (Genome, Genome){
        if random::<f64>()>CROSSOVER_RATE{
            return (self.clone(), genome.clone());   
        }

        let mut child1 = Genome::new();
        let mut child2 = Genome::new();

        let tp = random::<f64>() >= 0.5;

        if tp{
            //单点杂交: 从父母各取一个点，分别交换
            let pos1 = (random::<f64>()*self.length() as f64) as usize;
            let pos2 = (random::<f64>()*genome.length() as f64) as usize;
            for i in 0..pos1{
                child1.genes.push(self.genes[i]);
            }
            for i in 0..pos2{
                child2.genes.push(genome.genes[i]);
            }
            for i in pos1..self.length(){
                child2.genes.push(self.genes[i]);
            }
            for i in pos2..genome.length(){
                child1.genes.push(genome.genes[i]);
            }
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
            for i in 0..p11{
                child1.genes.push(self.genes[i]);
            }
            //中间段
            for i in p21..p22{
                child1.genes.push(genome.genes[i]);
            }
            //尾段
            for i in p12..self.length(){
                child1.genes.push(self.genes[i]);
            }

            //child2同上
            for i in 0..p21{
                child2.genes.push(genome.genes[i]);
            }
            for i in p11..p12{
                child2.genes.push(self.genes[i]);
            }
            for i in p22..genome.length(){
                child2.genes.push(genome.genes[i]);
            }
        }

        (child1, child2)
    }

    fn to_bf(&self) -> String {
        let mut bf = String::new();
        for c in &self.genes{
            bf.push(*c);
        }
        //println!("bf={}", bf);
        bf
    }

    fn run(&self) -> (String, u64, u64){
        let mut result = (String::new(), MAX_TICK, MAX_LOOP);
        let program = self.to_bf();
        if program.contains("[]"){
            return result;
        }
        if let Ok(block) = parser::parse(program.as_bytes()) {
            match panic::catch_unwind(||{
                let mut context = Context::new(MAX_TICK, MAX_LOOP);
                context.run(&block);
                (context.out, context.tick, context.loop_count)
            }){
                Ok((out, tick, loop_count)) =>{
                    result = (out, tick, loop_count);
                }
                Err(err) => println!("出错:{:?}", err)
            }
        }
        result
    }

    fn calc_fitness_add(out:&str, target:&str){
        // Adding
        // if (Int32.TryParse(_console.ToString(), out value))
        // {
        //     Fitness += 256 - Math.Abs(value - (input1 + input2));
        // }
    }

    fn calc_fitness_sub(){
        // // Subtracting
        // if (Int32.TryParse(_console.ToString(), out value))
        // {
        //     Fitness += 256 - Math.Abs(value - (input1 - input2));
        // }
    }

    //反转字符串适应分函数
    fn calc_fitness_reverse_string(out:&str, target:&str) -> f64{
        let mut fitness = 0.0;
        let out_bytes = out.as_bytes();
        let target = target.as_bytes();
        //匹配分
        for i in 0..target.len() {
            if out_bytes.len()>i{
                fitness += 255.0 - (out_bytes[i] as f64 - target[target.len()-i-1] as f64).abs();
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
        let (out, tick, loop_count) = self.run();

        //输出字符串计算适应分
        self.fitness = Genome::calc_fitness_print_string(&out, target);
        
        self.out = out;

        //执行时间越短， 适应分越高
        self.fitness -= tick as f64/35.0;
        self.fitness -= loop_count as f64/35.0;

        //指令越多适应分越低
        self.fitness -= self.to_bf().len() as f64/10.0;
    }
}

impl Clone for Genome {
    fn clone(&self) -> Genome {
        Genome{
            out: String::new(),
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
            println!("人口:{} 代数:{} 平均分:{} 最高分:{} out={:?} bf={}", self.populations.len(), self.generations, self.total_fitness/POPULATION_SIZE as f64, self.populations[0].fitness, out, self.populations[0].to_bf());
        }
        //新群体
        let mut new_pop = vec![];
        //精英选择
        for i in 0..NUM_ELITE{
            for _ in 0..NUM_COPIES_ELITE{
                new_pop.push(self.populations[i].clone());
            }
        }

        while new_pop.len() < POPULATION_SIZE{
            //每次生成两个孩子
            let mum = &self.populations[self.roulette_selection()];
            let dad = &self.populations[self.roulette_selection()];

            //杂交
            let (mut baby1, mut baby2) = mum.crossover(&dad);
            //变异
            baby1.mutate();
            baby1.mutate();
            //计算适应分
            //println!("baby1开始计算适应分 {}", baby1.to_bf());
            baby1.calc_fitness(&self.target);
            //println!("baby1适应分计算完成.");
            //println!("baby2开始计算适应分 {}", baby2.to_bf());
            baby2.calc_fitness(&self.target);
            //println!("baby2适应分计算完成.");

            new_pop.push(baby1);
            new_pop.push(baby2);
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
    let target = "Rust";
    //let target_fitness = target.len() as f64*255.0;
    let mut ga = GA::new(target);
    let now = Instant::now();
    while {
        let (out, tick, loop_count) = ga.populations[0].run();
        if out == target{
            let elapsed_ms = duration_to_milis(&now.elapsed());
            println!("最终程序:{} 适应分:{} 结果:{} 耗时:{}ms tick={}, loop={}", ga.populations[0].to_bf(), ga.populations[0].fitness, out, elapsed_ms, tick, loop_count);
            false
        }else{
            true
        }
    }{
        ga.epoch();
    }
}

pub fn duration_to_milis(duration: &Duration) -> f64 {
    duration.as_secs() as f64 * 1000.0 + duration.subsec_nanos() as f64 / 1_000_000.0
}
