
pub trait Fitness{
    fn base(&self) -> &BaseFitness;
    fn base_mut(&mut self) -> &mut BaseFitness;
    //程序源代码
    fn program(&self) -> &str;
    fn set_program(&mut self, program:&str);
    //程序运行以后的输出
    fn output(&self) -> &str;
    fn set_output(&mut self, output:&str);
    //真正的适应性分数. 用来确定解决方案是否找到
    fn fitness(&self) -> f64;
    fn set_fitness(&mut self, fitness:f64);
    //解决方案的目标适应性分数
    fn target_fitness(&self) -> f64;
    //指令执行数量
    fn ticks(&self) -> i64;
    fn set_ticks(&mut self, ticks: i64);
    //指令执行数量, 包含函数调用
    fn total_ticks(&self) -> i64;
    fn set_total_ticks(&mut self, ticks:i64);

    //设置:

    //包含功能的程序代码，将附加到主程序代码中。
    fn append_code(&self) -> &str;
    //儿童基因组将使用两个父母交叉的百分比机会。 默认值为0.7
    fn crossover_rate(&self) -> Option<f64>{ None }
    //儿童基因组突变基因的几率。 默认值0.01
    fn mutation_rate(&self) -> Option<f64>{ None }
    //生成的程序中的编程指令数（基因组阵列的大小）。循环）。 默认为100
    fn genome_size(&self) -> Option<usize> { None }
    //基因组可能增长的最大长度（仅在_expandAmount> 0时适用）。 默认为100
    fn max_genome_size(&self) -> Option<usize> { None }
    //程序在被杀死之前可以运行的最大迭代次数（防止无限循环）。 默认5000
    fn max_iteration_count(&self) -> Option<usize> { None }
    //每个_expandRate迭代（可能有助于学习），最大基因组大小将扩大此数量。 设置为0以禁用。 默认值为0
    fn expand_amount(&self) -> Option<isize>{ None }
    //最大的基因组大小将在这个世代间隔由_expandAmount扩展。 默认5000
    fn expand_rate(&self) -> Option<isize> { None }


    //获得权重的适应度。 将权重转换为程序代码，执行代码，对结果进行排名。
    fn get_fitness(&mut self, weights:Vec<f64>) -> f64;
    //运行程序源代码并将输出作为字符串返回以显示给用户。 将其与最终结果一起用于用户。
    fn run_program(&mut self, program:&str) -> &str;
    //返回实例化健身构造函数（不包括GA）所需的编译参数。
    //例子：
    // AddFitness：_maxIterationCount +“，”+ _trainingCount
    // StringFitness：_maxIterationCount +“，\”“+ _targetString +”\“”   
    // HelloUserFitness：_maxIterationCount +“，\”“+ _targetString +”\“，”+ _trainingCount
    fn get_constructor_parameters(&self) -> &str;
    //将目标适应度重置为0。
    fn reset_target_fitness(&mut self);
}

pub struct BaseFitness{
    //返回遗传算法的总体适应度（可能是变量，解决方案不是基于此值，仅基于排名）。
    fitness: f64,
    //目标健身达到。只在健身类的实例化中评估一次。
    target_fitness: f64,
    //程序在被杀死之前可以运行的最大迭代次数（防止无限循环）。
    max_iteration_count: isize,
    //要附加到程序的功能代码。
    append_functions: Option<String>,
    //由类用于收集控制台输出。
    console: String,
    //由类用于收集和连接输出以分配给Output。
    output: String,
}