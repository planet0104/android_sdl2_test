//指令集

//运算(38)  +-<>[]():0123456789*/asmd^&|~rl%@!xocg
//输出(2)   .n
//输入(2)   ,=
//其他(2)   q空格

//[f64]作为基因组 f64分为42份代表42个指令
//杂交: 单点杂交, 两点杂交
//变异: 插入并删除开头或者末尾, 删除并在开头或者末尾插入，循环左移/右移，替换

pub mod genome;
pub mod genalg;
use self::genome::Genome;

pub struct Params{
    pub pop_size: usize,
    pub chromo_length: usize,
    pub num_elite: usize,
    pub num_copies_elite: usize,
    pub num_thread: usize, //线程数
    pub mutation_rate: f64,  //变异率 0.05~0.3
    pub crossover_rate: f64, //杂交率 0.7
    pub calc_fitness: fn(genome:&Genome) -> f64,
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
            calc_fitness: self.calc_fitness,
        }
    }
}
