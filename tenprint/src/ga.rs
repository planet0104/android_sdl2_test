use rand::rngs::ThreadRng;
use std::sync::mpsc::channel;
use std::thread;
use std::sync::{Arc, Mutex};
use rand::{thread_rng, Rng};
use ::INSTRUCTION_SET;
use ::{PBrain, MAX_ITERATION_COUNT, calc_fitness};

pub struct Params{
    pub pop_size: usize,
    pub chromo_length: usize,
    pub num_elite: usize,
    pub num_copies_elite: usize,
    pub num_thread: usize, //线程数
    pub mutation_rate: f64,  //变异率 0.05~0.3
    pub crossover_rate: f64, //杂交率 0.7
}

impl Clone for Params {
    fn clone(&self) -> Params {
        Params {
            pop_size: self.pop_size,
            chromo_length: self.chromo_length,
            num_elite: self.num_elite,
            num_copies_elite: self.num_copies_elite,
            num_thread: self.num_thread,
            mutation_rate: self.mutation_rate,
            crossover_rate: self.crossover_rate,
        }
    }
}


//基因组
pub struct Genome {
    pub fitness: f64,
    genes: Vec<char>,
}

impl Genome {
    pub fn new(length: usize) -> Genome {
        let mut rng = thread_rng();
        let mut genes = vec![];
        for _ in 0..length {
            genes.push(*rng.choose(&INSTRUCTION_SET).unwrap());
        }
        Genome {
            fitness: 0.0,
            genes: genes,
        }
    }

    fn length(&self) -> usize{
        self.genes.len()
    }

    //变异
    pub fn mutate(&mut self, mutation_rate: f64) {
        let len = self.length();
        let mut rng = thread_rng();
        //每个基因进行变异
        for i in 0..len {
            if rng.gen::<f64>() < mutation_rate {
                // if rng.gen::<f64>() < 0.5 {
                //     //突变     
                //     self.genes[i] = *rng.choose(&INSTRUCTION_SET).unwrap();
                // }else{
                //     //旋转突变
                //     if rng.gen::<f64>() >= 0.5{
                //         self.genes.rotate_right(1);
                //     }else{
                //         self.genes.rotate_left(1);
                //     }
                // }
                self.genes[i] = *rng.choose(&INSTRUCTION_SET).unwrap();
            }
        }

        //随机替换一段
        if len>=20 && rng.gen::<f64>() < mutation_rate {
            //随机选择一个长度(较小)
            let splice_len = rng.gen_range(1, len/10);
            //println!("splice_len={}", splice_len);
            //随机选择一个点
            let p = rng.gen_range(0, len-splice_len);
            //生成一个或大或小的新基因片段
            self.genes.splice(
                p..p+splice_len,
                (0..rng.gen_range(0, splice_len * 2)).map(|_| *rng.choose(&INSTRUCTION_SET).unwrap()),
            );
        }
    }

    //杂交
    pub fn crossover(&self, genome: &Genome, crossover_rate: f64) -> (Genome, Genome) {
        let mut rng = thread_rng();

        if rng.gen::<f64>()>crossover_rate{
            return (self.clone(), genome.clone());   
        }

        let mut child1 = vec![];
        let mut child2 = vec![];

        let tp = rng.gen::<f64>() >= 0.5;

        if tp{
            //单点杂交: 从父母各取一个点，分别交换
            let p1 = rng.gen_range(0, self.length());
            let p2 = rng.gen_range(0, genome.length());
            
            child1.extend_from_slice(&self.genes[0..p1]);
            child1.extend_from_slice(&genome.genes[p2..genome.length()]);

            child2.extend_from_slice(&genome.genes[0..p2]);
            child2.extend_from_slice(&self.genes[p1..self.length()]);
        }else{
            //两点杂交: 从父母各取两个点，交换中间的部分
            let mut p11 = (rng.gen::<f64>()*self.length() as f64) as usize;
            let mut p12 = (rng.gen::<f64>()*self.length() as f64) as usize;
            if p11>p12{
                let tmp = p11;
                p11 = p12;
                p12 = tmp;
            }
            let mut p21 = (rng.gen::<f64>()*genome.length() as f64) as usize;
            let mut p22 = (rng.gen::<f64>()*genome.length() as f64) as usize;
            if p21>p22{
                let tmp = p21;
                p21 = p22;
                p22 = tmp;
            }
            //前段
            child1.extend_from_slice(&self.genes[0..p11]);
            //中间段
            child1.extend_from_slice(&genome.genes[p21..p22]);
            //尾段
            child1.extend_from_slice(&self.genes[p12..self.length()]);

            //child2同上
            child2.extend_from_slice(&genome.genes[0..p21]);
            child2.extend_from_slice(&self.genes[p11..p12]);
            child2.extend_from_slice(&genome.genes[p22..genome.length()]);
        }

        (Genome::from_genes(child1), Genome::from_genes(child2))
    }

    pub fn to_program(&self) -> String {
        let mut program = String::new();
        for c in &self.genes{
            program.push(*c);
        }
        program
    }

    pub fn get_output(&self) -> String{
        let program = self.to_program();
        let mut pbrain = PBrain::new(vec![], MAX_ITERATION_COUNT);
        let _ = pbrain.parse(program.chars());
        pbrain.output().clone()
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
    params: Params,
    pop: Vec<Genome>,
    total_fitness: f64,
    generations: usize,
    rng: ThreadRng,
}

impl GA {
    pub fn new(params:Params) -> GA {
        let mut pop = vec![];
        for _ in 0..params.pop_size{
            pop.push(Genome::new(params.num_copies_elite));
        }
        GA {
            params,
            generations: 0,
            total_fitness: 0.0,
            pop,
            rng: thread_rng(),
        }
    }

    //下一代
    pub fn epoch(&mut self){
        //计算总适应分
        self.total_fitness = 0.0;
        for p in &mut self.pop{
            self.total_fitness += p.fitness;
        }

        //按照得分排序
        self.pop.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());

        //新群体
        let mut new_pop = vec![];
        //精英选择
        let elite_count = self.params.num_copies_elite*self.params.num_elite;
        assert!(elite_count %2==0, "精英数量必须位偶数!");
        self.grab_n_best(self.params.num_elite, self.params.num_copies_elite, &mut new_pop);
        
        let (tx, rx) = channel();
        let new_pop = Arc::new(Mutex::new(new_pop));
        assert!((self.params.pop_size-elite_count) % self.params.num_thread == 0 , "(人口数-精英数量)必须为线程数量的整数倍!");
        let child_count = (self.params.pop_size-elite_count)/self.params.num_thread;
        assert!(child_count % 2 == 0 , "(人口数-精英数)量必须为偶数!");

        for _tid in 0..self.params.num_thread{
            let mut parents = vec![];
            for _ in 0..child_count/2{
                //每次生成两个孩子
                let n1 = self.get_chromo_roulette();
                let n2 = self.get_chromo_roulette();
                parents.push(self.pop[n1].clone());
                parents.push(self.pop[n2].clone());
            }
            let tx = tx.clone();
            let new_pop_clone = new_pop.clone();
            let params = self.params.clone();
            thread::spawn(move || {
                let mut childs = vec![];
                //每次生成两个孩子
                while childs.len()<child_count as usize{
                    let mum = parents.pop().unwrap();
                    let dad = parents.pop().unwrap();
                    //杂交
                    let (mut baby1, mut baby2) = mum.crossover(&dad, params.crossover_rate);
                    //变异
                    baby1.mutate(params.mutation_rate);
                    baby1.mutate(params.mutation_rate);
                    //计算适应分
                    baby1.fitness = calc_fitness(&baby1);
                    baby2.fitness =  calc_fitness(&baby2);

                    childs.push(baby1);
                    childs.push(baby2);
                }

                let mut new_pop = new_pop_clone.lock().unwrap();
                new_pop.append(&mut childs);
                
                if new_pop.len() == params.pop_size{
                    let mut pops = vec![];
                    pops.append(&mut new_pop);
                    tx.send(pops).unwrap();
                }
            });
        }

        //替换新的群体
        self.pop.clear();
        self.pop.append(&mut rx.recv().unwrap());
        self.generations += 1;
    }

    //精英选择
    pub fn grab_n_best(&self, num_elite: usize, num_copies: usize, pop:&mut Vec<Genome>){
        // for n_best in 0..num_elite{
        //     for _ in 0..num_copies{
        //         pop.push(self.pop[self.params.pop_size - 1 - n_best].clone())
        //     }
        // }
        for i in 0..num_elite{
            for _ in 0..num_copies{
                pop.push(self.pop[i].clone());
            }
        }
    }

    //赌轮选择
    pub fn get_chromo_roulette(&mut self) -> usize{
        // //产生一个0~总适应分的随机数字
        // let slice = self.rng.gen::<f64>() * self.total_fitness;
        // //放置被选择的基因组
        // let mut the_choose_one = 0;
        // //适应分累加
        // let mut fitness_so_far = 0.0;
        // for i in 0..self.params.pop_size{
        //     fitness_so_far += self.pop[i].fitness;
        //     //如果目前为止适应性分数大于随机数返回此点的基因组
        //     if fitness_so_far >= slice{
        //         the_choose_one = i;
        //         break;
        //     }
        // }
        // the_choose_one

        //生成0和总体适应分之间的随机数
        let slice = self.rng.gen::<f64>() * self.total_fitness;
        let mut fitness_so_far = 0.0;
        let mut the_choose_one = 0;
        for i in 0..self.pop.len(){
            fitness_so_far += self.pop[i].fitness;
            //如果当前适应分>随机数，返回此处的染色体
            if fitness_so_far > slice{
                the_choose_one = i;
                break;
            }
        }
        the_choose_one
    }

    pub fn get_chromos(&self) -> &Vec<Genome>{
        &self.pop
    }

    pub fn get_average_fitness(&self) -> f64{
        self.total_fitness/self.pop.len() as f64
    }
}