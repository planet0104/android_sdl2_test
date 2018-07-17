use ga::{Genome, CalcFitness};
use pbrain::{PBrain};
const MAX_ITERATION_COUNT: u64 = 1000;

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