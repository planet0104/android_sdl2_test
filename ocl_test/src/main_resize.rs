extern crate ocl;
extern crate lodepng;
use lodepng::RGB;
use lodepng::Bitmap;
use std::time::{Duration, Instant};
use ocl::ProQue;

const SRC:&'static str = r#"
        __kernel void count(__global float* buffer, float step) {
            int gid = get_global_id(0);
            float num = buffer[gid];
            float count = 0;
            float max = num + step;
            for (float n=num+1; n<=max; n++){
                if (n <= 3) {
                    if (n>1) count += 1;
                    continue;
                }
                
                bool b = false;
                for(int i=2;i<n;i++){
                    if((int)n%i == 0){
                        b = true;
                        break;
                    }
                }
                if (!b){
                    count += 1;
                }
            }
            buffer[gid] = count;
        }
    "#;

fn main() {
    let scale = 0.7;
    let ubuntu = lodepng::decode24_file("xinxing.png").unwrap();
    let mut images:Vec<&Bitmap<RGB<u8>>> = vec![];
    let image_count = 500;
    for _ in 0..image_count{
        images.push(&ubuntu);
    }
    println!("图片数量:{}", image_count);
    
    println!("CPU压缩:");
    let start_time = Instant::now();
        for i in 0..images.len(){
        let r = resize_cpu(&images[i], scale);
        if i%100==0 {
            println!("压缩进度{}%, 压缩后大小:{}x{} 字节:{}", ((i as f32/images.len() as f32)*100.0) as i32, r.1, r.2, r.0.len());
        }
    }
    println!("压缩耗时:{}ms", duration_to_milis(&start_time.elapsed()));

    let pro_que = ProQue::builder()
        .src(SRC)
        .dims(ubuntu.width,)
        .build().unwrap();

}

//GPU近邻插值缩放图像
// fn resize_gpu(src: &Bitmap<RGB<u8>>, width:usize, height: usize) -> Vec<RGB<u8>>{
//     vec![]
// }

//CPU近邻插值缩放图像
fn resize_cpu(src: &Bitmap<RGB<u8>>, scale: f32) -> (Vec<RGB<u8>>, usize, usize){
    let mut buffer = vec![];
    let (w0, h0) = (src.width, src.height);
    let (nw, nh) = (src.width as f32*scale, src.height as f32*scale);
    let fw = w0 as f32 / nw;
    let fh = h0 as f32 / nh;

    for y in 0..nh as usize{
        let y0 = (y as f32 * fh) as usize;
        for x in 0..nw as usize{
            let x0 = (x as f32 * fw) as usize;
            buffer.push(src.buffer[y0*w0+x0]);
        }
    }
    (buffer, nw as usize, nh as usize)
}

pub fn duration_to_milis(duration: &Duration) -> f64 {
    duration.as_secs() as f64 * 1000.0 + duration.subsec_nanos() as f64 / 1_000_000.0
}