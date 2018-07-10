use std::collections::HashMap;
use types::function::FunctionInst;

/// <summary>
    ///这是brainfuck解释器。
    ///
    ///>递增指针。
    /// <减少指针。
    /// +递增指针处的字节。
    /// - 减少指针处的字节。
    ///输出指针处的字节。
    ///，输入一个字节并将其存储在指针的字节中。
    /// [如果指针处的字节为零，则跳过匹配]。
    ///]向后跳转到匹配[除非指针处的字节为零。
    ///
    ///扩展命令，包含在BrainPlus中。
    /// @退出程序或在函数内部，返回主程序中的先前位置并恢复状态。
    /// $用指针处的字节覆盖存储器中的字节。
    ///！用存储中的字节覆盖指针处的字节。
    /// a，b调用函数a - z。
    /// 0-F将当前内存指针的值设置为16的倍数。
    /// *将函数的返回值设置为当前内存指针的值;父存储将获得返回值。
/// </ summary>

//用于交换函数调用状态的对象。 当函数终止时，将恢复此数据。
struct FnCallObj{
    instruction_pointer: i32,
    data_pointer: i32,
    function_input_pointer: i32,
    call_stack: Vec<i32>,
    exit_loop: bool,
    exit_loop_instruction_pointer: i32,
    ticks: i32,
    instruction: char,
    storage: u8,
    return_value: Option<u8>,
    max_iteration_count: i32
}

pub struct Interpreter{
    call_stack: Vec<i32>,
    input: Box<Fn()>,
    instruction_set: HashMap<char, Box<Fn()>>,
    //程序的内存
    memory: [u8; 32768],
    //输出函数
    output: Box<Fn()>,
    //程序代码
    source: Vec<char>,
    //数据指针
    data_pointer: i32,
    //指令指针
    instruction_pointer: i32,
    //布尔标志，指示我们是否应该跳过循环并继续执行下一个有效指令。 
    //如果指针为零并且读取了开始循环[指令，则在此情况下我们跳过匹配时使用]。
    exit_loop: bool,
    //保持循环开始的指令指针。 用于在搜索当前循环结束时绕过所有内循环。
    exit_loop_instruction_pointer: i32,
    //函数列表及其起始指令索引。
    functions: HashMap<char, FunctionInst>,
    //下一个功能的标识符。 将作为调用此函数的指令。
    next_function_character: char,
    //功能“调用堆栈”。
    function_call_stack: Vec<FnCallObj>,
    //指向当前调用堆栈的指针（m_FunctionCallStack或m_CallStack）。
    current_call_stack: Vec<i32>,
    function_input_pointer: i32,
    /// <summary>
    ///指向函数父内存的指针。 当从函数内执行输入（，）命令时，函数的当前存储器单元获取该指针处的父存储器值的副本。 这允许将多个值作为输入传递给函数。
    ///例如：++> ++++> + << a！。@，>， - [ - <+>] <+ $ @
    ///父内存包含：2,4,1。函数将包含：2,4并在存储中存储值6。 产生的父存储器保持为：2,4,1。在下一个命令！，父存储器将包含：6,4,1。然后将值6显示为输出。
    /// </ summary>
    

}