mod cbrain;
use cbrain::Interpreter;
use std::str;
use std::time::{Duration, Instant};

fn main() {
    let mut cbrain = Interpreter::new();
    let mut output = String::new();

    /*
    Hello程序:

    0 (<<<<< <<<<<)
    定义函数0: 单元格指针减去10( 本程序代表0Hello cbrain 0 中 移动到0Hell)
    > 72 > 101 > 7 o a c c +++ > 32 > 99 c - > 114 o - > 105 c > 5 a > 32 > 0
    >72:第1个单元格=72(H)、>101:第2个单元格=101(e)、>7:第3个单元格=7、o:复制第2个单元格到第4个单元格(101),定位到第4单元格、a: 3单元格=3单元格+4单元格=108,定位到3、
    0Hel
    c: 0Hell、c: 0Helll、+++: 0Hello、>32: 0Hello空格、>99: 0Hello c、c-: 0Hello cb、>114: 0Hello cbr、o: 复制b到下一个单元格变为a、>105: 0Hello cbrai
    c: 0Hello cbraii、>5: 0Hello cbraii5、a: 0Hello cbrain(n=i+5)、>32>0: 0Hello cbrain 0

    : <<< [. >] << [. <] 10 .
    调用函数0, 单元格指针指向0Hell、<<<:单元格指针执行0H、[.>]从H开始循环输出一直到0、<<:回退到空格之前、[.<]:循环倒序输出字母、10.：输出回车
    */

    let easter = str::from_utf8(include_bytes!("../easter.cb")).unwrap();
    let hello = str::from_utf8(include_bytes!("../hello.cb")).unwrap();
    let ascii = str::from_utf8(include_bytes!("../ascii.cb")).unwrap();
    //   {d,@,!,%,s,m,n}
    let now = Instant::now();
    cbrain.run(&mut vec![], &mut output, hello);
    cbrain.run(&mut vec![], &mut output, ascii);
    cbrain.run(&mut vec![2014], &mut output, easter);
    println!("耗时:{}ms", duration_to_milis(&now.elapsed()));
    println!("{}", output);
}

pub fn duration_to_milis(duration: &Duration) -> f64 {
    duration.as_secs() as f64 * 1000.0 + duration.subsec_nanos() as f64 / 1_000_000.0
}
