use math::Matrix2D;
use rand;
use rand::Rng;
use std::f32::consts::PI;

//线条包含两个顶点, 竖线
pub static LINE_VERTS:[(f32, f32); 2] = [(0.0, -1.0), (0.0, 1.0)];

#[derive(Debug, Copy, Clone)]
pub struct Line{
    pub verts:[(f32, f32); 2],
    pub verts_trans:[(f32, f32); 2],
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
    pub scale: f32,
}

impl Line{
    pub fn new() -> Line{
        Line{
            verts:LINE_VERTS,
            verts_trans: LINE_VERTS,
            x: 0.0, y:0.0, rotation:0.0, scale:1.0
            }
    }

    // pub fn verts(&self) -> &[(f32, f32)]{
    //     &self.verts
    // }

    pub fn world_transform(&mut self){
        //创建一个转换矩阵
        let mut matrix = Matrix2D::new_identity();
        //变比
        matrix.scale(self.scale, self.scale);
        //旋转
        matrix.rotate(self.rotation);
        //移动
        matrix.translate(self.x, self.y);
        //转换
        matrix.transform(&self.verts, &mut self.verts_trans);
    }

    pub fn verts_trans(&self) ->  &[(f32, f32)]{
        &self.verts_trans
    }

    //变异
    pub fn mutate(&mut self, width:u32, height:u32){
        let mut rng = rand::thread_rng();
        // match rng.gen_range(0, 4){
        //     0 => {
        //         //长度 - 最长变化10像素
        //         let dscale = rng.gen_range(-10.0, 10.0);
        //         self.scale += dscale;
        //     }
        //     1 => {
        //         //旋转 - 最大30度
        //         let dr = rng.gen_range(-PI/6.0, PI/6.0);
        //         self.rotation += dr;
        //     }
        //     2 => {
        //         //平移 - 最大10像素
        //         self.x += rng.gen_range(-10, 10) as f32;
        //         self.y += rng.gen_range(-10, 10) as f32;
        //     }
        //     _ => {
        //         //随机替换
        //         self.randomize(width, height)
        //     }
        // }
        match rng.gen_range(0, 4){
            0 => {
                //长度
                let dscale = rng.gen_range(-6.0, 6.0);
                self.scale += dscale;
                if self.scale<6.0{
                    self.scale = 6.0;
                }
            }
            1 => {
                //旋转
                let dr = rng.gen_range(-PI/9.0, PI/9.0);
                self.rotation += dr;
            }
            2 => {
                //平移 - 最大10像素
                self.x += rng.gen_range(-3, 3) as f32;
                self.y += rng.gen_range(-3, 3) as f32;
            }
            _ => {
                //随机替换
                //self.randomize(width, height)
            }
        }

        //长度 - 最长变化10像素
        // let dscale = rng.gen_range(-5.0, 5.0);
        // self.scale += dscale;
        //     //旋转 - 最大30度
        // let dr = rng.gen_range(-PI/8.0, PI/8.0);
        // self.rotation += dr;
        // //平移 - 最大10像素
        // self.x += rng.gen_range(-5, 5) as f32;
        // self.y += rng.gen_range(-5, 5) as f32;
    }

    //随机线条
    pub fn randomize(&mut self, width:u32, height:u32){
        let mut rng = rand::thread_rng();
        //随机位置
        let (x, y) = (rng.gen_range(0, width), rng.gen_range(0, height));
        self.x = x as f32;
        self.y = y as f32;
        //随机大小
        self.scale = rng.gen_range(15, 25) as f32;
        //随机角度 +=90度
        self.rotation = rng.gen_range(-PI/2.0, PI/2.0);
    }
}