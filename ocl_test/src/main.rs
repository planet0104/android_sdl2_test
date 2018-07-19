extern crate ocl;
use ocl::ProQue;
use std::time::{Duration, Instant};
extern crate rand;
use rand::{thread_rng, Rng};
use ocl::Platform;
use ocl::enums::PlatformInfo;

//测试
const SIZE:usize = 45000000 * 1;

fn check_prime_gpu(pro_que: &ProQue, array:&Vec<f32>, result: &mut Vec<f32>) -> ocl::Result<()> {
    let buffer = pro_que.create_buffer::<f32>()?;
    buffer.cmd().write(array).enq().unwrap();

    let kernel = pro_que.kernel_builder("div")
        .arg(&buffer)
        .build()?;

    unsafe { kernel.enq()?; }
    buffer.read(result).enq()?;
    Ok(())
}

fn main(){
    let src = r#"
        __kernel void div(__global float* buffer) {
            int gid = get_global_id(0);
            int n = (int)buffer[gid];
            if (n <= 3) {
                if (n>1){
                    buffer[gid] = 1.0;
                }else{
                    buffer[gid] = 0.0;
                }
            } else if (n % 2 == 0 || n % 3 == 0) {
                buffer[gid] = 0.0;
            } else {
                int i = 5;
                while (i*i <= n){
                    if (n % i == 0 || n % (i + 2) == 0) {
                        buffer[gid] = 0.0;
                        return;
                    }
                    i += 6;
                }
                buffer[gid] = 1.0;
            }
        }
    "#;

    let platforms = Platform::list();
    for p in platforms{
        println!("设备: {:?}", p.info(PlatformInfo::Version));
    }

    let pro_que = ProQue::builder()
        .src(src)
        .dims(SIZE)
        .build().unwrap();

    println!("max_wg_size={:?}", pro_que.max_wg_size());

    //随机生成一个质数数组
    let mut rng = thread_rng();
    let start_time = Instant::now();
    let mut numbers:Vec<f32> = vec![];
    for _ in 0..SIZE{
        numbers.push(rng.gen_range(100000, 1001000) as f32);
    }
    let mut result = vec![0.0; SIZE];
    println!("生成数组 耗时:{}ms", duration_to_milis(&start_time.elapsed()));
    //println!("{:?}", numbers);
    //下面代码判断数组中的数是否是质数，如果是质数将对应位置设置为1，否则设置为0

    //cpu判断是否是质数
    let start_time = Instant::now();
    check_prime_cpu(&mut numbers, &mut result);
    println!("{:?} 耗时:{}ms", result.get(0..5), duration_to_milis(&start_time.elapsed()));
    
    //gpu判断是否质数
    let start_time = Instant::now();
    check_prime_gpu(&pro_que, &mut numbers, &mut result).unwrap();
    println!("{:?} 耗时:{}ms", result.get(0..5), duration_to_milis(&start_time.elapsed()));
}

fn check_prime_cpu(array:&Vec<f32>, result: &mut Vec<f32>){
    for i in 0..array.len(){
        result[i] = if is_prime(array[i] as u32){
            1.0
        }else{
            0.0
        };
    }
}

//判断是否是质数
fn is_prime(n: u32) -> bool{
    if n <= 3 {
        return n > 1;
    } else if n % 2 == 0 || n % 3 == 0 {
        return false;
    } else {
        let mut i = 5;
        while i*i <= n{
            if n % i == 0 || n % (i + 2) == 0 {
                return false;
            }
            i += 6;
        }
        return true;
    }
}

pub fn duration_to_milis(duration: &Duration) -> f64 {
    duration.as_secs() as f64 * 1000.0 + duration.subsec_nanos() as f64 / 1_000_000.0
}