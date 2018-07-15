/**
    pbrain编程语言的解释器（具有程序功能的Brainf ** k）
    http://www.parkscomputing.com/applications/pbrain/

    语法与传统的Brainf ** k相同，具体如下


    Brainf-ck由以下指令集组成：

1   >   递增指针。
2   <   减少指针。
3   +   递增指针处的字节。
3   -   减少指针处的字节。
5   .   输出指针处的字节。
6   ,   输入一个字节并将其存储在指针的字节中。
7   [   如果指针处的字节为零，则跳过匹配]。
8   ]   向后跳转到匹配[除非指针处的字节为零。

    符号添加：

    （
    开始程序

    ）
    结束程序

    ：
    由当前位置的值标识的呼叫过程


    程序由数字ID标识：

    +（[ - ]）

    假设当前位置为零，则定义编号1的程序，程序执行后将当前位置置零。

    ++（<< [>> + << - ]> [> + < - ]>）

    假设当前位置为零，则定义一个编号为2的程序，接受两个参数。它将参数1和参数2相加并将和存储在当前位置, 并将参数1和2归零。

    +++（[ - ]> ++++++++++ [<++++> - ] <++++++++> [ - ] ++:.）

    假设当前位置为零，则定义打印函数3，输出当前位置代表的0~9之间的ASCII字符。

    +++> +++++> ++：
    调用过程2，传入参数3和5。

    所有上述示例可以组合到下面的程序中。注意
    程序编号为1,2和3，因为当前位置是
    在每个过程定义之前递增。

    +（[ - ]）
    +（ - ：<< [>> + << - ]> [> + < - ]>）
    +（[ - ]> ++++++++++ [<++++> - ] <++++++++> [ - ] ++：。）
    > +++> +++++> ++：
    > +++：

    报告错误情况，对stderr和a进行简短诊断从可执行文件返回的错误号。
    解释器抛出的错误如下：
    1 - 内存不足
    2 - 未知程序
    3 - 内存地址超出范围
    4 - 无法找到匹配]开始[
    999 - 未知异常
 */

use std::collections::HashMap;

//定义内存数组中包含的类型
type PbrainMemType = i32;

//使用动态数组存储内存位置。
type Mem = Vec<PbrainMemType>;

//用于存储指令串的类型; 用于程序和循环
type SourceBlock = Vec<char>;
type Procedures = HashMap<PbrainMemType, Vec<char>>;

pub struct PBrain{
    mp: usize, //内存指针
    mem: Mem,
    procedures: Procedures,
    output: String, //输出
    input: Vec<PbrainMemType>,
    iteration_count: u64,
    max_iteration_count: u64,
    ins_count: u64,
}

impl PBrain{
    pub fn new(input: Vec<PbrainMemType>, max_iteration_count: u64) -> PBrain{
        PBrain{
            mp: 0,
            mem: vec![0],
            procedures: Procedures::new(),
            output: String::new(),
            input: input,
            iteration_count: 0,
            max_iteration_count,
            ins_count: 0,
        }
    }

    pub fn output(&self) ->&String{
        &self.output
    }

    pub fn parse<I>(&mut self, iter: I) -> Result<(), String> where I: Iterator<Item=char>{
        self.ins_count = 0;
        let mut source_block = SourceBlock::new();
        //将指令从输入流复制到源块
        for ii in iter{
            source_block.push(ii);
        }
        //执行源块中的指令
        self.interpret(&source_block)
    }

    fn interpret(&mut self, block: &SourceBlock) -> Result<(), String>{
        let mut ii = 0;
        let eos = block.len();

        //限制最大迭代次数, 防止进入死循环
        if self.iteration_count > self.max_iteration_count{
            return Err(PBrain::get_error(6));
        }
        self.iteration_count += 1;

        while ii<eos{
            self.ins_count += 1;
            match block[ii]{
                '+' => self.mem[self.mp] += 1,
                '-' => self.mem[self.mp] -= 1,
                '>' => {
                    self.mp += 1;
                    if self.mp == self.mem.len() {
                        self.mem.resize(self.mp + 64, 0);
                    }
                }
                '<' => {
                    if self.mp == 0{
                        return Err(PBrain::get_error(3));
                    }
                    self.mp -= 1;
                }
                '.' => {
                    //self.output.push(from_u32(self.mem[self.mp] as u32).unwrap_or(' '));
                    self.output.push(self.mem[self.mp] as u8 as char);
                }
                ',' => {
                    if let Some(i) = self.input.pop(){
                        self.mem[self.mp] = i;
                    }else{
                        return Err(PBrain::get_error(5));
                    }
                }
                '[' => {
                    //转到循环中的第一条指令
                    ii += 1;
                    {
                        let mut nbest = 0;
                        let begin = ii;
                        //找到匹配的 ]
                        while ii != eos{
                            if block[ii] == '[' {
                                nbest += 1;
                            }else if block[ii] == ']' {
                                if nbest != 0 {
                                    nbest -= 1;
                                }else {
                                    break;
                                }
                            }
                            ii+=1;
                        }

                        //如果在源块中找不到 ] 匹配项，则报告错误。
                        if ii == eos {
                            return Err(PBrain::get_error(4));
                        }

                        //此时迭代器将指向匹配的]字符，该字符是在循环中处理的指令范围的末尾之后的一个指令。
                        let r = self._loop(&block.get(begin..=ii).unwrap().to_vec());
                        if !r.is_ok(){
                            return r;
                        }
                    }

                },

                '(' => {
                    ii += 1;
                    {
                        let mut source_block = SourceBlock::new();

                        while ii != eos && block[ii] != ')' {
                            source_block.push(block[ii]);
                            ii += 1;
                        }
                        self.procedures.insert(self.mem[self.mp], source_block);
                    }
                }
                ':' => {
                    //查找与当前位置的值匹配的源块。 如果找到，执行它。
                    let procedure = { if let Some(p) = self.procedures.get(&self.mem[self.mp]){
                            Some(p.clone())
                    }else { None}};

                    if procedure.is_none(){
                        return Err(PBrain::get_error(2));
                    }

                    let r = self.interpret(&procedure.unwrap());
                    if !r.is_ok(){
                        return r;
                    }
                }
                _ => ()
            }

            ii += 1;
        }

        Ok(())
    }

    // pub fn iteration_count(&self) -> u64{
    //     self.iteration_count
    // }

    //执行的指令个数
    pub fn instruction_count(&self) -> u64{
        self.ins_count
    }

    pub fn _loop(&mut self, block: &SourceBlock) -> Result<(), String>{
        //解释指令，直到当前存储单元中的值为零
        let mut result = Ok(());
        while self.mem[self.mp] !=0 {
            result = self.interpret(&block);
            if result.is_err(){
                break;
            }
        }
        result
    }

    /*
    1 - 内存不足
    2 - 未知程序
    3 - 内存地址超出范围
    4 - 无法找到匹配]开始[
    */
    pub fn get_error(code: i32) -> String{
        match code{
            1 => "内存溢出".to_string(),
            2 => "未知函数".to_string(),
            3 => "内存地址超出范围".to_string(),
            4 => "[和]不匹配".to_string(),
            5 => "没有数据输入".to_string(),
            6 => "超过最大迭代次数".to_string(),
            _ => "未知错误".to_string()
        }
    }
}