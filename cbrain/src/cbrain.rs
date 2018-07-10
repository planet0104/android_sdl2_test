/*
from bf
Operation 	Meaning
+ 	increment value in current cell
- 	decrement value in current cell
> 	advance data cell index up 1
< 	retreat data cell index down 1
[ 	if current cell zero, scan source index forward to next ]
] 	if current cell non-zero, scan source index backwards to previous [
. 	output current cell as character value
, 	accept key value to current cell

from pbrain
Operation 	Meaning
( 	define procedure using current cell value as label
) 	end procedure definition
 : 	evaluate procedure with label matching current cell value

cbrain adds
Operation 	Meaning
digits, 0 though 9 	if last operation was digit, scale current cell up by 10, otherwise zero out and add in digit
{ 	scan forward through source text to position past balanced nested close brace }
* 	multiply current value by 10
/ 	integer divide current value by 10
a 	add current cell into previous cell and retreat
s 	subtract current cell from previous cell and retreat
m 	multiply current cell by previous cell into previous cell and retreat
d 	divide previous cell by current cell, leaving quotient in previous and remainder in current. No retreat.
^ 	bitwise xor current cell with previous cell and retreat
& 	bitwise and current cell with previous cell and retreat
| 	bitwise or current cell with previous cell and retreat
~ 	bitwise not of current cell into current cell
r 	right shift previous cell by current cell bits and retreat
l 	left shift previous cell by current cell bits and retreat
 % 	compute modulus of previous cell by current cell into previous cell and retreat
@ 	fetch current cell value and place in memory register
 ! 	place content of memory register in current cell
x 	exchange current cell with previous
o 	copy previous cell over to next and advance
c 	copy current cell to next cell and advance
n 	output current cell as integer
# 	output current cell as formatted integer
b 	output current cell as base 2
= 	input integer and place value in current cell
" 	scan forward to ", call that COBOL module, passing current and then replacing with RETURN-CODE
t 	toggle step trace
 ? 	display current memory cells
g 	goback
q 	quit
help 	reserved word, calls "cbrainhelp", the OpenCOBOL module using cbrain quoted string.
file 	reserved word, loads script using rest of source text as filename. 
*/

const SIZE: usize = 65536;
const PROCEDURES: usize = 256;

pub struct Interpreter {
    //sizie
    a: Vec<isize>, //单元格
    memreg: isize,

    //i32
    s: Vec<usize>, // 8x65536 字节
    sp: usize,
    ptable: Vec<isize>, //过程指针表 8x256字节
    t: Vec<usize>,      // 8x65536字节
    p: usize,           //单元格指针
    q: usize,           //代码指针
    length: usize,
    tmp: isize,
    scale: i32,
    tracer: i32,
    tracing: i32,

    //char
    code: Vec<char>, //65536字节
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            a: vec![0; SIZE],
            memreg: 0,
            s: vec![0; SIZE],
            sp: 0,
            ptable: vec![0; PROCEDURES],
            t: vec![0; SIZE],
            p: 0,
            q: 0,
            length: 0,
            tmp: 0,
            scale: 0,
            tracer: 0,
            tracing: 0,
            code: vec![' '; SIZE],
        }
    }

    pub fn run(&mut self, input: &mut Vec<isize>, output: &mut String, program: &str) -> isize {
        self.length = program.len();
        let bytes = program.as_bytes();

        for i in 0..self.length {
            self.code[i] = bytes[i] as char;
        }

        self.scale = 0;
        self.q = 0;
        while self.q < self.length {
            match self.code[self.q] {
                '(' | '[' => {
                    //存储过程起始指针
                    self.s[self.sp] = self.q;
                    self.sp += 1;
                }
                ')' => {
                    if self.sp == 0 {
                        self.e(7);
                        return 0;
                    }
                    self.sp -= 1;
                    if self.code[self.s[self.sp]] != '(' {
                        self.e(7);
                        return 0;
                    }
                    //存储过程结束指针
                    self.t[self.s[self.sp]] = self.q;
                }
                ']' => {
                    if self.sp == 0 {
                        self.e(5);
                        return 0;
                    }
                    self.sp -= 1;
                    self.t[self.s[self.sp]] = self.q;
                    self.t[self.q] = self.s[self.sp];
                    if self.code[self.s[self.sp]] != '[' {
                        self.e(5);
                        return 0;
                    }
                }
                _ => (),
            }

            self.q += 1;
        }

        if self.sp != 0 {
            self.sp -= 1;
            self.e(if self.code[self.s[self.sp]] == '[' {
                4
            } else {
                6
            });
        }

        for q in 0..PROCEDURES {
            self.ptable[q] = -1;
        }

        self.q = 0;
        while self.q < self.length {
            if self.tracer != 0 && self.code[self.q] == ' ' {
                self.tmp = self.code[self.q] as isize;
                eprintln!(
                    "[{} at {} was {}, ",
                    self.tmp as u8 as char, self.p, self.a[self.p]
                );
                self.tracing = 1;
            }
            match self.code[self.q] {
                /* bf */
                '+' => {
                    self.a[self.p] += 1;
                    self.scale = 0;
                }
                '-' => {
                    self.a[self.p] -= 1;
                    self.scale = 0;
                }
                '<' => {
                    if self.p == 0 {
                        self.e(3);
                        self.p = 0;
                        return 0;
                    }
                    self.p -= 1;
                    self.scale = 0;
                }
                '>' => {
                    self.p += 1;
                    if self.p >= SIZE {
                        self.e(3);
                        self.p = 0;
                        return -1;
                    }
                    self.scale = 0;
                }
                '.' => {
                    output.push(if self.a[self.p] == 10 {
                        '\n'
                    } else {
                        self.a[self.p] as u8 as char
                    });
                    self.scale = 0;
                }
                ',' => {
                    if let Some(ch) = input.pop() {
                        let ch = ch as u8 as char;
                        self.a[self.p] = if ch == '\n' { 10 } else { ch as isize };
                    }
                    self.scale = 0;
                }
                '[' => {
                    if self.a[self.p] == 0 {
                        self.q = self.t[self.q];
                    }
                    self.scale = 0;
                }
                ']' => {
                    if self.a[self.p] != 0 {
                        self.q = self.t[self.q];
                    }
                    self.scale = 0;
                }
                /* pbrain */
                '(' => {
                    //ptable[单元格的内容]=当前代码指针
                    self.ptable[self.a[self.p] as usize] = self.q as isize;
                    //跳到过程结束
                    self.q = self.t[self.q];
                    //println!("1>self.q={} {}", self.q, self.code[self.q]);
                    self.scale = 0;
                }
                ')' => {
                    self.sp -= 1;
                    self.q = self.s[self.sp];
                    //println!("2>self.q={} {}", self.q, self.code[self.q]);
                    self.scale = 0;
                }
                ':' => {
                    //evaluate procedure with label matching current cell value
                    self.s[self.sp] = self.q; //代码指针入栈
                    self.sp += 1; //堆栈指针+1
                                  //判断指针是否为空
                    if self.ptable[self.a[self.p] as usize] < 0 {
                        self.e(2);
                        return 0;
                    }
                    self.q = self.ptable[self.a[self.p] as usize] as usize; //取代码指针
                    self.scale = 0;
                }
                /* cbrain */
                '{' => {
                    self.tmp = 1;
                    while self.tmp != 0 && self.q < self.length {
                        self.q += 1;
                        if self.code[self.q] == '}' {
                            self.tmp -= 1;
                        }
                        if self.code[self.q] == '{' {
                            self.tmp += 1;
                        }
                    }
                    if self.tmp != 0 {
                        self.e(9);
                        return 0;
                    }
                    self.scale = 0;
                }
                '}' => {
                    self.e(10);
                    self.scale = 0;
                    return 0;
                }
                '*' => {
                    self.a[self.p] *= 10;
                    self.scale = 0;
                }
                '/' => {
                    self.a[self.p] /= 10;
                    self.scale = 0;
                }
                '&' => {
                    if self.p < 1 {
                        self.e(3);
                        return 0;
                    }
                    self.p -= 1;
                    self.a[self.p] &= self.a[self.p + 1];
                    self.scale = 0;
                }
                '|' => {
                    if self.p < 1 {
                        self.e(3);
                        return 0;
                    }
                    self.p -= 1;
                    self.a[self.p] |= self.a[self.p + 1];
                    self.scale = 0;
                }
                '^' => {
                    if self.p < 1 {
                        self.e(3);
                        return 0;
                    }
                    self.p -= 1;
                    self.a[self.p] ^= self.a[self.p + 1];
                    self.scale = 0;
                }
                '~' => {
                    self.a[self.p] = !self.a[self.p];
                    self.scale = 0;
                }
                'a' => {
                    //add current cell into previous cell and retreat
                    //当前单元格值加到前一个单元格，指针减去1
                    if self.p < 1 {
                        self.e(3);
                        return 0;
                    }
                    self.p -= 1;
                    self.a[self.p] += self.a[self.p + 1];
                    self.scale = 0;
                }
                's' => {
                    if self.p < 1 {
                        self.e(3);
                        return 0;
                    }
                    self.p -= 1;
                    self.a[self.p] -= self.a[self.p + 1];
                    self.scale = 0;
                }
                'm' => {
                    if self.p < 1 {
                        self.e(3);
                        return 0;
                    }
                    self.p -= 1;
                    self.a[self.p] *= self.a[self.p + 1];
                    self.scale = 0;
                }
                'd' => {
                    if self.p < 1 {
                        self.e(3);
                        return 0;
                    }
                    if self.a[self.p] != 0 {
                        self.tmp = self.a[self.p - 1];
                        self.a[self.p - 1] /= self.a[self.p];
                        self.a[self.p] = self.tmp - self.a[self.p] * self.a[self.p - 1];
                    }
                    self.scale = 0;
                }
                '%' => {
                    if self.p < 1 {
                        self.e(3);
                        return 0;
                    }
                    //compute modulus of previous cell by current cell into previous cell and retreat
                    self.p -= 1;
                    self.a[self.p] %= self.a[self.p + 1];
                    self.scale = 0;
                }
                'r' => {
                    if self.p < 1 {
                        self.e(3);
                        return 0;
                    }
                    self.p -= 1;
                    self.a[self.p] >>= self.a[self.p + 1];
                    self.scale = 0;
                }
                'l' => {
                    if self.p < 1 {
                        self.e(3);
                        return 0;
                    }
                    self.p -= 1;
                    self.a[self.p] <<= self.a[self.p + 1];
                    self.scale = 0;
                }
                '0' => {
                    //if last operation was digit, scale current cell up by 10, otherwise zero out and add in digit
                    self.a[self.p] *= if self.scale > 0 { 10 } else { 0 };
                    self.scale += 1;
                }
                '1' => {
                    self.a[self.p] *= if self.scale > 0 { 10 } else { 0 };
                    self.scale += 1;
                    self.a[self.p] += 1;
                }
                '2' => {
                    self.a[self.p] *= if self.scale > 0 { 10 } else { 0 };
                    self.scale += 1;
                    self.a[self.p] += 2;
                }
                '3' => {
                    self.a[self.p] *= if self.scale > 0 { 10 } else { 0 };
                    self.scale += 1;
                    self.a[self.p] += 3;
                }
                '4' => {
                    self.a[self.p] *= if self.scale > 0 { 10 } else { 0 };
                    self.scale += 1;
                    self.a[self.p] += 4;
                }
                '5' => {
                    self.a[self.p] *= if self.scale > 0 { 10 } else { 0 };
                    self.scale += 1;
                    self.a[self.p] += 5;
                }
                '6' => {
                    self.a[self.p] *= if self.scale > 0 { 10 } else { 0 };
                    self.scale += 1;
                    self.a[self.p] += 6;
                }
                '7' => {
                    self.a[self.p] *= if self.scale > 0 { 10 } else { 0 };
                    self.scale += 1;
                    self.a[self.p] += 7;
                }
                '8' => {
                    self.a[self.p] *= if self.scale > 0 { 10 } else { 0 };
                    self.scale += 1;
                    self.a[self.p] += 8;
                }
                '9' => {
                    self.a[self.p] *= if self.scale > 0 { 10 } else { 0 };
                    self.scale += 1;
                    self.a[self.p] += 9;
                }
                '=' => {
                    //读入一个整数替换当前单元格
                    if let Some(i) = input.pop() {
                        self.a[self.p] = i;
                    }
                    self.scale = 0;
                }
                'n' | '#' | 'b' => {
                    //输出当前单元格的整数
                    output.push_str(&format!("{}", self.a[self.p]));
                }
                'x' => {
                    //交换当前和前一个单元格
                    self.tmp = self.a[self.p];
                    self.a[self.p] = self.a[self.p - 1];
                    self.a[self.p - 1] = self.tmp;
                    self.scale = 0;
                }
                'c' => {
                    //copy current cell to next cell and advance
                    //当前单元格的值复制到下一个单元格，指针+1
                    if self.p >= SIZE {
                        self.e(3);
                        return 0;
                    }
                    self.p += 1;
                    self.a[self.p] = self.a[self.p - 1];
                    self.scale = 0;
                }
                'o' => {
                    //copy previous cell over to next and advance
                    if self.p < 1 || self.p >= SIZE {
                        self.e(3);
                        return 0;
                    }
                    self.p += 1;
                    self.a[self.p] = self.a[self.p - 2];
                    self.scale = 0;
                }
                '@' => {
                    //fetch current cell value and place in memory register
                    self.memreg = self.a[self.p];
                    self.scale = 0;
                }
                '!' => {
                    //place content of memory register in current cell
                    self.a[self.p] = self.memreg;
                    self.scale = 0;
                }
                '"' => {}
                '?' => {
                    //display current memory cells
                    let mut cell = String::new();
                    self.tmp = self.p as isize;
                    while self.tmp >= 0 {
                        cell.push_str(&format!(
                            "[{} is {}]\n",
                            self.tmp, self.a[self.tmp as usize]
                        ));
                        self.tmp -= 1;
                    }
                    println!("单元格内容:\n{}", cell);
                    self.scale = 0;
                }
                'O' => {
                    //display current memory cells
                    let mut cell = String::new();
                    for p in 0..=self.p {
                        let v = self.a[p];
                        if v >= 32 && v <= 126 {
                            cell.push_str(&format!("{}", self.a[p] as u8 as char));
                        } else {
                            cell.push_str(&format!("{}", self.a[p]));
                        }
                    }
                    println!(
                        "单元格内容:\n{}\n单元格指针:{}\n代码指针:{}",
                        cell, self.p, self.q
                    );
                    self.scale = 0;
                }
                't' => {
                    //toggle step trace
                    self.tracer = if self.tracer != 0 { 0 } else { 1 };
                    self.scale = 0;
                }
                'g' => return (self.p + 1) as isize,
                'q' => return -1,
                _ => self.scale = 0,
            }

            if self.tracing != 0 && !(self.code[self.q] == ' ') {
                eprintln!(" after {} is {}]\n", self.p, self.a[self.p]);
                self.tracing = 0;
            }

            self.q += 1;
        }

        self.p as isize + 1
    }

    fn e(&self, i: i32) {
        match i {
            2 => eprintln!(
                "调用未定义的程序 ({}) with {} at {} of cbrain",
                self.a[self.p], self.p, self.q
            ),
            3 => eprintln!(
                "pointer too far {} at {} of cbrain",
                if self.p > 0 { "right" } else { "left" },
                self.q
            ),
            4 => eprintln!("unmatched '[' at byte {} of cbrain", self.s[self.sp]),
            5 => eprintln!("unmatched ']' at byte {} of cbrain", self.q),
            6 => eprintln!("unmatched '(' at byte {} of cbrain", self.s[self.sp]),
            7 => eprintln!("unmatched ')' at byte {} of cbrain", self.q),
            8 => eprintln!("can't open cbrain"),
            9 => eprintln!("unmatched '｛' at byte {} of cbrain", self.s[self.sp]),
            10 => eprintln!("unmatched '｝' at byte {} of cbrain", self.q),
            11 => eprintln!("unmatched '\"' at byte {} of cbrain", self.q),
            12 => eprintln!("divide by zero at byte {} of cbrain", self.q),
            _ => {}
        }
    }
}
