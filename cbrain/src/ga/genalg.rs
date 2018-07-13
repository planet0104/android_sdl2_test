
use ga::genome::Genome;
use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};
use std::sync::mpsc::channel;
use std::thread;
use std::sync::{Arc, Mutex};
use ga::Params;

pub struct GenAlg {
    params: Params,
    pop: Vec<Genome>,
    total_fitness: f64,
    fittest_genome: usize,
    best_fitness: f64,
    worst_fitness: f64,
    average_fitness: f64,
    rng: ThreadRng,
}

impl GenAlg {
    pub fn new(params: Params) -> GenAlg {
        let mut pop = vec![];
        for _ in 0..params.pop_size {
            let mut genome = Genome::new(params.chromo_length);
            //计算适应分
            let fitness = (params.calc_fitness)(&genome);
            genome.fitness = fitness;
            pop.push(genome);
        }

        GenAlg{
            params,
            rng: thread_rng(),
            pop,
            total_fitness: 0.0,
            fittest_genome: 0,
            best_fitness: 0.0,
            worst_fitness: ::std::f64::MAX,
            average_fitness: 0.0,
        }
    }

    //代循环
    pub fn epoch(&mut self) -> &Vec<Genome>{
        //重置变量
        self.reset();
        //升序排序(用户变比和精英选择)
        self.pop.sort_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap());
        self.calculate_best_worst_av_tot();
        let mut new_pop = vec![];
        //选择精英
        let elite_count = self.params.num_copies_elite*self.params.num_elite;
        assert!(elite_count %2==0, "精英数量必须位偶数!");
        self.grab_n_best(self.params.num_elite, self.params.num_copies_elite, &mut new_pop);
        
        let (tx, rx) = channel();
        let new_pop = Arc::new(Mutex::new(new_pop));
        assert!((self.params.pop_size-elite_count) % self.params.num_thread == 0 , "(人口数-精英数量)必须为线程数量的整数倍!");
        let child_count = (self.params.pop_size-elite_count)/self.params.num_thread;
        assert!(child_count % 2 == 0 , "(人口数-精英数)量必须为偶数!");
        for _tid in 0..self.params.num_thread{
            let mut parents:Vec<Genome> = vec![];
            for _ in 0..child_count/2{
                //每次选择两个父代生成两个子代
                let (i1,i2) = (self.get_chromo_roulette(), self.get_chromo_roulette());
                parents.push(self.pop[i1].clone());
                parents.push(self.pop[i2].clone());
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
                    baby1.fitness = (params.calc_fitness)(&baby1);
                    baby2.fitness = (params.calc_fitness)(&baby2);

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
        &self.pop
    }

    fn reset(&mut self){
        self.total_fitness = 0.0;
        self.best_fitness = 0.0;
        self.worst_fitness = ::std::f64::MAX;
        self.average_fitness = 0.0;
    }

    //精英选择
    pub fn grab_n_best(&self, n: usize, num_copies: usize, pop:&mut Vec<Genome>){
        for n_best in 0..n{
            for _ in 0..num_copies{
                pop.push(self.pop[self.params.pop_size - 1 - n_best].clone())
            }
        }
    }

    //赌轮选择
    pub fn get_chromo_roulette(&mut self) -> usize{
        //产生一个0~总适应分的随机数字
        let slice = self.rng.gen::<f64>() * self.total_fitness;
        //放置被选择的基因组
        let mut the_choose_one = 0;
        //适应分累加
        let mut fitness_so_far = 0.0;
        for i in 0..self.params.pop_size{
            fitness_so_far += self.pop[i].fitness;
            //如果目前为止适应性分数大于随机数返回此点的基因组
            if fitness_so_far >= slice{
                the_choose_one = i;
            }
        }
        the_choose_one
    }

    //适应性分数变比排名，将人群按适应性分数的升序排序，随后根据它的位置给出一个适应性分数.
    pub fn fitness_scale_rank(&mut self){
        let fitness_multiplier = 1.0;
        for i in 0..self.params.pop_size{
            self.pop[i].fitness = i as f64 * fitness_multiplier;
        }
        //重新计算选择时的参数
        self.calculate_best_worst_av_tot();
    }

    //计算最佳、最差、平均分和总分
    pub fn calculate_best_worst_av_tot(&mut self){
        self.total_fitness = 0.0;
        let mut highest_so_far = 0.0;
        let mut lowest_so_far = ::std::f64::MAX;
        for i in 0..self.pop.len(){
            let fit = self.pop[i].fitness;
            //更新最佳分数
            if fit > highest_so_far{
                highest_so_far = fit;
                self.fittest_genome = i;
                self.best_fitness = highest_so_far;
            }
            //更新最差分数
            if fit < lowest_so_far{
                lowest_so_far = fit;
                self.worst_fitness = lowest_so_far;
            }
            self.total_fitness += fit;
        }
        self.average_fitness = self.total_fitness/self.params.pop_size as f64;
    }

    pub fn get_chromos(&self) -> &Vec<Genome>{
        &self.pop
    }

    pub fn get_average_fitness(&self) -> f64{
        self.total_fitness/self.params.pop_size as f64
    }

    pub fn get_best_fitness(&self) -> f64{
        self.best_fitness
    }
}