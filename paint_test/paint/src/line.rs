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

    pub fn verts(&self)
}