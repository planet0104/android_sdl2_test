mod cbrain;
use cbrain::Interpreter;
use std::str;
use std::time::{Duration, Instant};

fn main() {
    let mut cbrain = Interpreter::new();
    let mut output = String::new();

    let easter = str::from_utf8(include_bytes!("../easter.cb")).unwrap();
    let hello = str::from_utf8(include_bytes!("../hello.cb")).unwrap();
    let ascii = str::from_utf8(include_bytes!("../ascii.cb")).unwrap();

    cbrain.run(&mut vec![], &mut output, hello);
    cbrain.run(&mut vec![], &mut output, ascii);
    cbrain.run(&mut vec![2014], &mut output, easter);
    
    println!("{}", output);
}

//let now = Instant::now();
//println!("耗时:{}ms", duration_to_milis(&now.elapsed()));
pub fn duration_to_milis(duration: &Duration) -> f64 {
    duration.as_secs() as f64 * 1000.0 + duration.subsec_nanos() as f64 / 1_000_000.0
}
