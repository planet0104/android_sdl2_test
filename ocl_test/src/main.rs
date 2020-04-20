use rand::Rng;
use ocl::ProQue;
use std::time::{Duration, Instant};

//dll转lib
//.\pexports.exe C:\Windows\SysWOW64\opencl.dll >opencl.def
//.\lib.exe /def:C:\Users\JiaYe\vcpkg\downloads\tools\perl\c\bin\opencl.def /machine:X64

const SIZE:usize = 200_000_000;

fn main(){
    println!("数组大小:{}", SIZE);
    let start_time = Instant::now();
    let mut m_array = vec![];
    let mut a_array = vec![];

    let mut rng = rand::thread_rng();
    for _ in 0..SIZE{
        m_array.push(rng.gen::<f32>());
        a_array.push(rng.gen::<f32>());
    }
    println!("数组始化完成. 耗时{}ms", duration_to_milis(&start_time.elapsed()));

    //cpu
    let start_time = Instant::now();
    let mut total = 0;
    for i in 0..SIZE{
        total += m_array[i].log(a_array[i]) as i64;
    }
    println!("CPU计算完成. 总和:{} 耗时{}ms", total, duration_to_milis(&start_time.elapsed()));

    let start_time = Instant::now();
    let src = r#"
        __kernel void count(__global const float* m, __global const float* a, __global float* result) {
            int idx = get_global_id(0);
            result[idx] = log(m[idx])/log(a[idx]);
        }
    "#;

    let pro_que = ProQue::builder()
        .src(src)
        .dims(SIZE)
        .build().unwrap();
    println!("ProQue创建完成. 耗时{}ms", duration_to_milis(&start_time.elapsed()));

    let start_time = Instant::now();
    let buffer_m = pro_que.create_buffer::<f32>().unwrap();
    buffer_m.cmd().write(&m_array).enq().unwrap();
    let buffer_a = pro_que.create_buffer::<f32>().unwrap();
    buffer_a.cmd().write(&a_array).enq().unwrap();
    let buffer_result = pro_que.create_buffer::<f32>().unwrap();

    println!("buffer创建完成. 耗时{}ms", duration_to_milis(&start_time.elapsed()));

    let start_time = Instant::now();
    let kernel = pro_que.kernel_builder("count")
    .arg(&buffer_m)
    .arg(&buffer_a)
    .arg(&buffer_result)
    .build().unwrap();

    unsafe { kernel.enq().unwrap(); }

    let mut result:Vec<f32> = vec![0.0; SIZE];
    buffer_result.read(&mut result).enq().unwrap();

    println!("GPU计算完成. 耗时{}ms", duration_to_milis(&start_time.elapsed()));

    let start_time = Instant::now();
    let mut total = 0;
    for c in result{
        total += c as i64;
    }
    println!("统计完成. 总和:{} 耗时{}ms", total, duration_to_milis(&start_time.elapsed()));
}

pub fn duration_to_milis(duration: &Duration) -> f64 {
    duration.as_secs() as f64 * 1000.0 + duration.subsec_nanos() as f64 / 1_000_000.0
}