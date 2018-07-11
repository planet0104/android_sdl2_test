use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};

//基因组
pub struct Genome {
    genes: Vec<f64>,
    pub fitness: f64,
    rng: ThreadRng,
}

impl Genome {
    pub fn new(length: usize) -> Genome {
        let mut rng = thread_rng();
        let mut genes = vec![];
        for _ in 0..length {
            genes.push(rng.gen());
        }
        Genome {
            rng,
            fitness: 0.0,
            genes: genes,
        }
    }

    pub fn from_genes(genes: Vec<f64>) -> Genome {
        Genome {
            rng: thread_rng(),
            fitness: 0.0,
            genes: genes,
        }
    }

    //杂交
    pub fn crossover(&mut self, genome: &Genome, crossover_rate: f64) -> (Genome, Genome) {
        if self.rng.gen::<f64>() > crossover_rate {
            (self.clone(), genome.clone())
        } else {
            //由于两个基因组长度不一致, 在两个基因组中选择一段(长度不超过较短的基因组)基因进行交换。
            let (mut baby1, baby1_len, mut baby2, baby2_len) = if self.length() < genome.length() {
                (
                    self.genes.clone(),
                    self.length(),
                    genome.genes.clone(),
                    genome.length(),
                )
            } else {
                (
                    genome.genes.clone(),
                    genome.length(),
                    self.genes.clone(),
                    self.length(),
                )
            };

            //选择交叉点
            let m1 = self.rng.gen_range(0, baby1_len);
            let m2 = self.rng.gen_range(m1, baby1_len);
            let len = m2 - m1;
            let d1 = self.rng.gen_range(0, baby2_len - len);
            //交换
            baby1[m1..len].swap_with_slice(&mut baby2[d1..len]);

            (Genome::from_genes(baby1), Genome::from_genes(baby2))
        }
    }

    //变异
    pub fn mutate(&mut self, mutation_rate: f64) {
        let len = self.length();

        //每个基因进行变异
        for i in 0..len {
            if self.rng.gen::<f64>() < mutation_rate {
                if self.rng.gen::<f64>() < 0.5 {
                    //突变     
                    self.genes[i] = self.rng.gen();
                }else{
                    //旋转突变
                    if self.rng.gen::<f64>() >= 0.5{
                        self.genes.rotate_right(1);
                    }else{
                        self.genes.rotate_left(1);
                    }
                }
            }
        }

        //随机替换一段
        if self.rng.gen::<f64>() < mutation_rate {
            let p1 = self.rng.gen_range(0, len);
            let p2 = self.rng.gen_range(p1, len);
            //生成一个或大或小的新基因片段
            let mut rng = self.rng.clone();
            self.genes.splice(
                p1..p2,
                (0..self.rng.gen_range(0, (p2 - p1) * 2)).map(|_| rng.gen()),
            );
        }
    }

    //基因组转换成代码
    pub fn to_program(&self) -> String{
        let mut program = String::new();
        let chars = b"+-<>[]():0123456789*/asmd^&|~rl%@!xocg.n,=q ";
        for gene in &self.genes{
            let i = (gene/(1.0/(chars.len()-1) as f64)).round() as usize;
            program.push(chars[i] as char);
        }
        program
    }

    pub fn length(&self) -> usize {
        self.genes.len()
    }
}

impl Clone for Genome {
    fn clone(&self) -> Genome {
        Genome {
            rng: self.rng.clone(),
            fitness: self.fitness,
            genes: self.genes.clone(),
        }
    }
}
