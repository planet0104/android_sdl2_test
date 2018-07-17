use ga::{Genome, CalcFitness};
use pbrain::{PBrain};

const MAX_ITERATION_COUNT:u64 = 1000;

//反转字符串
pub struct AddFitness{
    inputs: Vec<Vec<i32>>,
    targets: Vec<i32>
}
impl Clone for AddFitness{
    fn clone(&self) -> AddFitness {
        AddFitness{
            inputs: self.inputs.clone(),
            targets: self.targets.clone()
        }
    }
}
impl AddFitness{
    pub fn new() -> AddFitness{
        //输入和目标输出
        AddFitness{
            inputs: vec![vec![1, 2],
                            vec![3, 4],
                            vec![5, 1],
                            vec![6, 2],
                            vec![3, 6],
                            vec![2, 0]],
            targets: vec![3, 7, 6, 8, 9, 2]
        }
    }

    fn run_program(&self, genome:&Genome) -> Vec<i32>{
        let program = genome.to_program();
        let input:Vec<i32> = vec![6, 7];
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

impl CalcFitness for AddFitness{
    //检查输出是否达到目标
    fn is_fitness_achieved(&self, genome:&Genome) -> bool{
        let output = self.run_program(genome);
        output == &[13]
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
                    input[index] as i32
                }else{
                    penalty += 1.0;
                    0
                }
            }), MAX_ITERATION_COUNT);
            let _ = pbrain.parse(program.chars());
            
            //输出字符串计算适应分
            let output = pbrain.output();

            //匹配分
            if output.len()>0{
                fitness += 256.0 - (output[0] as f64 - self.targets[i] as f64).abs();
            }
            fitness -= output.len() as f64 - 1.0;

            fitness -= penalty;

            //执行指令数量越短， 适应分越高
            //fitness -= pbrain.instruction_count() as f64 / 1000.0;

            //指令越多适应分越低
            //fitness -= program.len() as f64/2000.0;
        }
        fitness
    }
}