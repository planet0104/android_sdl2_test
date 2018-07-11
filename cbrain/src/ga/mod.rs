//指令集

//运算(38)  +-<>[]():0123456789*/asmd^&|~rl%@!xocg
//输出(2)   .n
//输入(2)   ,=
//其他(2)   q空格

//[f64]作为基因组 f64分为42份代表42个指令
//杂交: 单点杂交, 两点杂交
//变异: 插入并删除开头或者末尾, 删除并在开头或者末尾插入，循环左移/右移，替换

pub mod genome;
use self::genome::Genome;

pub struct GenAlg {
    pop: Vec<Genome>,
    pop_size: usize,     //人口数
    mutation_rate: f64,  //变异率 0.05~0.3
    crossover_rate: f64, //杂交率 0.7
}

impl GenAlg {
    pub fn new(
        pop_size: usize,
        mutation_rate: f64,
        crossover_rate: f64,
        chromo_length: usize,
    ) -> GenAlg {
        let mut pop = vec![];
        for _ in 0..pop_size {
            pop.push(Genome::new(chromo_length));
        }

        GenAlg{
            pop_size,
            pop,
            mutation_rate,
            crossover_rate
        }
    }

    //精英选择
    pub fn grab_n_best(&self, n: usize, num_copies: usize, pop:&mut Vec<Genome>){
        for ibest in 0..n{
            for _ in 0..num_copies{
                pop.push(self.pop[self.pop_size-1- ibest].clone())
            }
        }
    }

    //适应性分数变比排名，将人群按适应性分数的升序排序，随后根据它的位置给出一个适应性分数.
    pub fn fitness_scale_rank(&mut self){
        let fitness_multiplier = 1.0;
        for i in 0..self.pop_size{
            self.pop[i].fitness = i as f64 * fitness_multiplier;
        }
        //重新计算选择时的参数

    }

    pub fn 
}
