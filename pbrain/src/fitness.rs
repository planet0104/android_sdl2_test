use ga::{Genome, CalcFitness};
use pbrain::{PBrain};
use ::MAX_ITERATION_COUNT;
use rand::{thread_rng, Rng};


//反转字符串
pub struct SortFitness{
    inputs: Vec<Vec<i32>>,
    targets: Vec<Vec<i32>>
}
impl Clone for SortFitness{
    fn clone(&self) -> SortFitness {
        SortFitness{
            inputs: self.inputs.clone(),
            targets: self.targets.clone()
        }
    }
}
impl SortFitness{
    pub fn new() -> SortFitness{
        SortFitness{
            inputs: vec![vec![9, 8, 9, 6, 4],
                            vec![0, 8, 2, 2, 4],
                            vec![7, 0, 2, 5, 5],
                            vec![3, 4, 6, 3, 6],
                            vec![8, 5, 9, 2, 2]],
            targets: vec![vec![4, 6, 8, 9, 9],
                            vec![0, 2, 2, 4, 8],
                            vec![0, 2, 5, 5, 7],
                            vec![3, 3, 4, 6, 6],
                            vec![2, 2, 5, 8, 9]]
        }
    }
}

impl CalcFitness for SortFitness{
    fn is_fitness_achieved(&self, genome:&Genome) -> bool{
        let program = genome.to_program();
        let input = [9, 8, 9, 6, 4];
        let mut pbrain = PBrain::new(Box::new(move |index|->i32{
            if index<input.len(){
                input[index]
            }else{
                0
            }
        }), MAX_ITERATION_COUNT);
        let _ = pbrain.parse(program.chars());
        let output = pbrain.output();
        output == &[4, 6, 8, 9, 9]
    }

    fn get_output(&self, genome: &Genome) -> String{
        let program = genome.to_program();
        let input = [9, 8, 9, 6, 4];
        let mut pbrain = PBrain::new(Box::new(move |index|->i32{
            if index<input.len(){
                input[index]
            }else{
                0
            }
        }), MAX_ITERATION_COUNT);
        let _ = pbrain.parse(program.chars());
        pbrain.output_i32()
    }

    fn calc_fitness(&self, genome: &Genome) -> f64{
        let mut fitness = 0.0;
        let program = genome.to_program();

        let mut penalty = 0.0;

        for i in 0..self.inputs.len(){
            let input = self.inputs[i].clone();
            let mut pbrain = PBrain::new(Box::new(move |index|->i32{
                if index<input.len(){
                    input[index]
                }else if index==input.len(){
                    0
                }else{
                    penalty += 1.0;
                    0
                }
            }), MAX_ITERATION_COUNT);
            let _ = pbrain.parse(program.chars());
            
            //输出字符串计算适应分
            let output = pbrain.output();

            //匹配分
            let target = &self.targets[i];
            for i in 0..target.len() {
                if output.len()>i{
                    fitness += 10.0 - (output[i] as f64 - target[i] as f64).abs();
                }
            }

            fitness -= penalty;

            //执行时间越短， 适应分越高
            fitness -= pbrain.instruction_count() as f64 / 500.0;

            //指令越多适应分越低
            fitness -= program.len() as f64/2000.0;
        }
        fitness
    }
}




//输出字符串 的适应函数
pub struct PrintFitnessFunction{
    target: String
}
impl PrintFitnessFunction{
    pub fn new(target: &str) -> PrintFitnessFunction{
        PrintFitnessFunction{
            target: target.to_string()
        }
    }
}
impl Clone for PrintFitnessFunction {
    fn clone(&self) -> PrintFitnessFunction {
        PrintFitnessFunction{
            target: self.target.clone()
        }
    }
}
impl CalcFitness for PrintFitnessFunction{
    fn is_fitness_achieved(&self ,genome:&Genome) -> bool{
        self.get_output(&genome) == self.target
    }

    fn get_output(&self, genome: &Genome) -> String{
        let program = genome.to_program();
        let mut pbrain = PBrain::new(Box::new(|_i|->i32{255}), MAX_ITERATION_COUNT);
        let _ = pbrain.parse(program.chars());
        pbrain.output_ascii()
    }
    fn calc_fitness(&self, genome: &Genome) -> f64{
        let mut fitness = 0.0;
        let program = genome.to_program();
        let mut pbrain = PBrain::new(Box::new(|_i|->i32{255}), MAX_ITERATION_COUNT);
        let target = self.target.as_bytes();
        if let Ok(()) = pbrain.parse(program.chars()){
            //输出字符串计算适应分
            let ascii = pbrain.output_ascii();
            let out_bytes = ascii.as_bytes();
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
        }else{
            fitness -= 255.0*target.len() as f64;
        }

        //执行时间越短， 适应分越高
        fitness -= pbrain.instruction_count() as f64 / 500.0;

        //指令越多适应分越低
        fitness -= program.len() as f64/10.0;

        fitness
    }
}