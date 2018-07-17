use ga::{Genome, CalcFitness};
use pbrain::{PBrain};

const MAX_ITERATION_COUNT:u64 = 1000;

//反转字符串
pub struct RevertStringFitness{
    inputs: Vec<Vec<char>>,
    targets: Vec<Vec<char>>
}
impl Clone for RevertStringFitness{
    fn clone(&self) -> RevertStringFitness {
        RevertStringFitness{
            inputs: self.inputs.clone(),
            targets: self.targets.clone()
        }
    }
}
impl RevertStringFitness{
    pub fn new() -> RevertStringFitness{
        //输入和目标输出
        RevertStringFitness{
            inputs: vec![vec!['a'],
                            vec!['Y', 'e'],
                            vec!['J', 'i', 'a'],
                            vec!['p', 'l', 'a', 'n', 'e', 't'],
                            vec!['r', 'u', 's', 't']],
            targets: vec![vec!['e', 'Y'],
                            vec!['e', 'Y'],
                            vec!['a', 'i', 'J'],
                            vec!['t', 'e', 'n', 'a', 'l', 'p'],
                            vec!['t', 's', 'u', 'r']]
        }
    }
}

impl CalcFitness for RevertStringFitness{
    //检查输出是否达到目标
    fn is_fitness_achieved(&self, genome:&Genome) -> bool{
        let output = self.get_output(genome);
        output == "!uoY roF dooG"
    }

    //执行过程中检测输出
    fn get_output(&self, genome: &Genome) -> String{
        let program = genome.to_program();
        let input:Vec<i32> = "Good For You!".chars().map(|c| c as i32).collect();
        let mut pbrain = PBrain::new(Box::new(move |index|->i32{
            if index<input.len(){
                input[index]
            }else{
                0
            }
        }), MAX_ITERATION_COUNT);
        let _ = pbrain.parse(program.chars());
        pbrain.output_ascii()
    }

    fn calc_fitness(&self, genome: &Genome) -> f64{
        let mut fitness = 0.0;
        let program = genome.to_program();

        let mut penalty = 0.0;

        for i in 0..self.inputs.len(){
            let input = self.inputs[i].clone();
            let mut pbrain = PBrain::new(Box::new(move |index|->i32{
                if index<input.len(){
                    input[index] as i32
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
                    fitness += 255.0 - (output[i] as f64 - target[i] as u8 as f64).abs();
                }
            }
            fitness -= (output.len() as f64 - target.len() as f64).abs();

            fitness -= penalty;

            //执行指令数量越短， 适应分越高
            fitness -= pbrain.instruction_count() as f64 / 200.0;

            //指令越多适应分越低
            fitness -= program.len() as f64/800.0;
        }
        fitness
    }
}