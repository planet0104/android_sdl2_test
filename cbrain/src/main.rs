extern crate rand;
mod cbrain;
mod ga;
use cbrain::Interpreter;
use std::str;
use std::time::{Duration, Instant};
use ga::genalg::GenAlg;
use ga::genome::Genome;
use ga::Params;
use std::panic;

fn main() {
    
    fn calc_fitness(genome:&Genome) -> f64{
        let mut fitness = 0.0;
        let program = genome.to_program();

        let mut p = 0;
        let mut iteration_count = 0;
        let mut output = String::new();

        match panic::catch_unwind(||{
                let mut output = String::new();
                let mut cbrain = Interpreter::new();
                let p = cbrain.run(&mut vec![], &mut output, &program, 5000);
                (p, cbrain.iteration_count(), output)
            }){
                Ok((c, ic, o)) =>{
                    p = c;
                    iteration_count = ic;
                    output = o;
                }
                Err(_err) => ()
            }
        
        if p == 0{
            fitness = -99999.0;
        }else{
            let out_bytes = output.as_bytes();
            let target = b"Hello!";
            //匹配分
            for i in 0..target.len() {
                if out_bytes.len()>i{
                    fitness += 255.0 - (out_bytes[i] as f64 - target[i] as f64).abs();
                }
            }
            if out_bytes.len()>target.len(){
                //超出target的, 减分
                for _ in target.len()..out_bytes.len(){
                    fitness -= 5.0;
                }
            }

            //执行时间越短， 适应分越高
            fitness -= iteration_count as f64/35.0;

            //指令越多适应分越低
            fitness -= program.len() as f64/10.0;
        }
        //println!("program={}\nfitness={}", program, fitness);
        fitness
    }

    let mut genalg = GenAlg::new(Params{
        pop_size: 100,
        chromo_length: 200,
        num_elite: 2,
        num_copies_elite: 1,
        num_thread: 7,
        mutation_rate: 0.2,
        crossover_rate: 0.7,
        calc_fitness: calc_fitness,
    });

    for i in 0..1000{
        println!("gen={} 最高分:{}", i, genalg.get_best_fitness());
        genalg.epoch();
    }
    let program = genalg.get_chromos()[99].to_program();
    let mut output = String::new();
    let mut cbrain = Interpreter::new();
    cbrain.run(&mut vec![], &mut output, &program, 5000);
    println!("{} out={}", genalg.get_chromos()[99].to_program(), output);


    // let mut cbrain = Interpreter::new();
    // let mut output = String::new();
    // //let hello = "72.101.108.108.111.33.[]";
    // let hello = "7(c47|+q]0x**|4oc@-/n594 -s5sr:/cc-cs%70xl3^,46n";
    // cbrain.run(&mut vec![], &mut output, hello, 5000);
    // println!("{}", output);
}

//let now = Instant::now();
//println!("耗时:{}ms", duration_to_milis(&now.elapsed()));
pub fn duration_to_milis(duration: &Duration) -> f64 {
    duration.as_secs() as f64 * 1000.0 + duration.subsec_nanos() as f64 / 1_000_000.0
}
