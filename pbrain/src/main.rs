extern crate rand;

mod pbrain;
mod ga;
mod fitness;
use std::time::{Duration, Instant};
use fitness::PrintFitnessFunction;
use fitness::SortFitness;
use ga::{GA, Params, CalcFitness};

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

//const INSTRUCTION_SET:[char; 11] = ['>', '<', '+', '-', '.', ',', '[', ']', '(', ')', ':'];
const INSTRUCTION_SET:[char; 9] = ['>', '<', '.', ',', '[', ']', '(', ')', ':'];
const MUTATION_RATE: f64 = 0.02;//0.05~0.3
const CROSSOVER_RATE: f64 = 0.6;//0.7
const INITIAL_GENOME_SIZE: usize = 200;
const NUM_ELITE: usize = 3;//精英选择个数
const NUM_COPIES_ELITE: usize = 2; //每个精英复制数
const NUM_THREAD: usize = 8;//线程数
const POPULATION_SIZE: usize = 30*NUM_THREAD+NUM_ELITE*NUM_COPIES_ELITE;//人口数量
const MAX_ITERATION_COUNT:u64 = 1000;

fn main() {

    let fitness = PrintFitnessFunction::new("Hi!");
    let fitness = SortFitness::new();
    
    let mut ga = GA::new(Params{
        chromo_length: INITIAL_GENOME_SIZE,
        crossover_rate: CROSSOVER_RATE,
        mutation_rate: MUTATION_RATE,
        num_copies_elite: NUM_COPIES_ELITE,
        num_elite: NUM_ELITE,
        num_thread: NUM_THREAD,
        pop_size: POPULATION_SIZE,
        fitness_calc: Box::new(fitness.clone())
    });
    let start_time = Instant::now();
    let mut counter = Instant::now();
    let mut generations = 0;
    while {
        if fitness.is_fitness_achieved(&ga.get_chromos()[0]){
            let elapsed_ms = duration_to_milis(&start_time.elapsed());
            let out = fitness.get_output(&ga.get_chromos()[0]);
            println!("generations={} 最终程序:{} 适应分:{} 结果:{} 耗时:{}ms", generations, ga.get_chromos()[0].to_program(), ga.get_chromos()[0].fitness, out, elapsed_ms);
            false
        }else{
            true
        }
    }{
        ga.epoch();
        generations += 1;
        let elapsed_ms = duration_to_milis(&counter.elapsed());
        if elapsed_ms>=1000.0{
            counter = Instant::now();
            let chromos = ga.get_chromos();
            let out = fitness.get_output(&chromos[0]);
            println!("人口:{} 代数:{} 平均分:{} 最高分:{} out={:?} program={}", chromos.len(), generations, ga.get_average_fitness(), chromos[0].fitness, out, chromos[0].to_program());
        }
    }
}

pub fn duration_to_milis(duration: &Duration) -> f64 {
    duration.as_secs() as f64 * 1000.0 + duration.subsec_nanos() as f64 / 1_000_000.0
}
