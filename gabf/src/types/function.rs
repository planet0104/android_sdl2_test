pub trait Function{
    ///控制函数如何从父存储器读取输入（，）：在当前存储器数据指针处或从存储器的起始处读取。
    ///如果为true，将从父级的位置0读取输入。 这意味着，无论父级当前的内存数据位置如何，父读取的第一个输入值将是函数获得的第一个输入值。 这可能使GA更容易运行该函数，因为在调用函数之前它不需要精确的内存位置。
    ///如果为false（默认值），将从父项的当前内存数据位置读取输入。 这意味着，如果父级已将内存指针向上移动3个插槽，则该功能将从位置3的内存开始读取。
    fn read_input_at_momory_start(&self) -> bool;
    fn set_read_input_at_momory_start(&mut self, b: bool);

    //自定义最大迭代计数函数。
    fn max_iteration_count(&self) -> i32;
    fn set_max_iteration_count(&mut self, count:i32);
}

///解释器的功能特定设置。 包括指令指针值。
pub struct FunctionInst{
    read_input_at_momory_start: bool,
    max_iteration_count: i32,
    instruction_pointer: i32,
}

impl FunctionInst{
    pub fn instruction_pointer(&self) -> i32{
        self.instruction_pointer
    }
}

impl Function for FunctionInst{
    fn read_input_at_momory_start(&self) -> bool{
        self.read_input_at_momory_start
    }
    fn set_read_input_at_momory_start(&mut self, b: bool){
        self.read_input_at_momory_start = b;
    }

    //自定义最大迭代计数函数。
    fn max_iteration_count(&self) -> i32{
        self.max_iteration_count
    }
    fn set_max_iteration_count(&mut self, count:i32){
        self.max_iteration_count = count;
    }
}