use ga::{Genome, CalcFitness};
use pbrain::{PBrain};

const MAX_ITERATION_COUNT:u64 = 1000;

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
        //输入和目标输出
        SortFitness{
            inputs: vec![vec![1],
                            vec![5,4],
                            vec![2,6,1],
                            vec![3,6,2,5,7],
                            vec![7,3,1,67,9,4]],
            targets: vec![vec![1],
                            vec![4,5],
                            vec![1, 2, 6],
                            vec![2,3,5,6,7],
                            vec![1,3,4,7,9,67]]
        }
    }

    fn run_program(&self, genome: &Genome) -> Vec<i32>{
        let program = genome.to_program();
        let input:Vec<i32> = vec![1,4,8,3,7,2,9];
        let mut pbrain = PBrain::new(Box::new(move |index|->i32{
            if index<input.len(){
                input[index]
            }else{
                0
            }
        }), MAX_ITERATION_COUNT);
        let _ = pbrain.parse(program.chars());
        pbrain.output().clone()
    }
}

impl CalcFitness for SortFitness{
    //检查输出是否达到目标
    fn is_fitness_achieved(&self, genome:&Genome) -> bool{
        let output = self.run_program(genome);
        output == &[1,2,3,4,7,8,9]
    }

    //执行过程中检测输出
    fn get_output(&self, genome: &Genome) -> String{
        let output = self.run_program(genome);
        format!("{:?}", output)
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
                    fitness += 50.0 - (output[i] as f64 - target[i] as f64).abs();
                }
            }
            fitness -= (output.len() as f64 - target.len() as f64).abs() * 10.0;

            fitness -= penalty;

            //执行指令数量越短， 适应分越高
            fitness -= pbrain.instruction_count() as f64 / 200.0;

            //指令越多适应分越低
            fitness -= program.len() as f64/800.0;
        }
        fitness
    }
}