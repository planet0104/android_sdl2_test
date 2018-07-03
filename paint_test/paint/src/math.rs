pub struct Matrix{
    _11: f32, _12: f32, _13: f32,
    _21: f32, _22: f32, _23: f32,
    _31: f32, _32: f32, _33: f32,
}
impl Matrix{
    pub fn new()->Matrix{
         Matrix{
                _11: 0.0, _12: 0.0, _13: 0.0,
                _21: 0.0, _22: 0.0, _23: 0.0,
                _31: 0.0, _32: 0.0, _33: 0.0,
            }
    }
}

pub struct Matrix2D{
    matrix: Matrix,
}

impl Matrix2D{
    pub fn new() -> Matrix2D{
        Matrix2D{
            matrix: Matrix::new()
        }
    }

    pub fn new_identity() -> Matrix2D{
        let mut m = Matrix2D::new();
        m.identity();
        m
    }

    //创建单位矩阵
    pub fn identity(&mut self){
        self.matrix._11 = 1.0; self.matrix._12 = 0.0; self.matrix._13 = 0.0;
		self.matrix._21 = 0.0; self.matrix._22 = 1.0; self.matrix._23 = 0.0;
		self.matrix._31 = 0.0; self.matrix._32 = 0.0; self.matrix._33 = 1.0;
    }

    pub fn multiply(&mut self, m_in:Matrix){
        let mut tmp = Matrix::new();
		//第一行
		tmp._11 = (self.matrix._11*m_in._11) + (self.matrix._12*m_in._21) + (self.matrix._13*m_in._31);
		tmp._12 = (self.matrix._11*m_in._12) + (self.matrix._12*m_in._22) + (self.matrix._13*m_in._32);
		tmp._13 = (self.matrix._11*m_in._13) + (self.matrix._12*m_in._23) + (self.matrix._13*m_in._33);
		//第二行
		tmp._21 = (self.matrix._21*m_in._11) + (self.matrix._22*m_in._21) + (self.matrix._23*m_in._31);
		tmp._22 = (self.matrix._21*m_in._12) + (self.matrix._22*m_in._22) + (self.matrix._23*m_in._32);
		tmp._23 = (self.matrix._21*m_in._13) + (self.matrix._22*m_in._23) + (self.matrix._23*m_in._33);
		//第三行
		tmp._31 = (self.matrix._31*m_in._11) + (self.matrix._32*m_in._21) + (self.matrix._33*m_in._31);
		tmp._32 = (self.matrix._31*m_in._12) + (self.matrix._32*m_in._22) + (self.matrix._33*m_in._32);
		tmp._33 = (self.matrix._31*m_in._13) + (self.matrix._32*m_in._23) + (self.matrix._33*m_in._33);

        self.matrix = tmp;
    }

    //创建变换矩阵
    pub fn translate(&mut self, x:f32, y:f32){
        let mat = Matrix{
            _11: 1.0, _12: 0.0, _13: 0.0,
            _21: 0.0, _22: 1.0, _23: 0.0,
            _31: x, _32: y, _33: 1.0,
        };
        self.multiply(mat);
    }

    //创建变比矩阵
    pub fn scale(&mut self, x_scale:f32, y_scale:f32){
        let mat = Matrix{
            _11: x_scale, _12: 0.0, _13: 0.0,
            _21: 0.0, _22: y_scale, _23: 0.0,
            _31: 0.0, _32: 0.0, _33: 1.0,
        };
        self.multiply(mat);
    }

    //创建旋转矩阵
    pub fn rotate(&mut self, rot: f32){
        let sin = rot.sin();
        let cos = rot.cos();
        let mat = Matrix{
            _11: cos,  _12: sin, _13: 0.0,
            _21: -sin, _22: cos, _23: 0.0,
            _31: 0.0,  _32: 0.0, _33: 1.0,
        };
        self.multiply(mat);
    }

    pub fn transform(&self, points:&[(f32, f32)], trans:&mut[(f32, f32)]){
        for i in 0..points.len() {
			let x = (self.matrix._11*points[i].0) + (self.matrix._21*points[i].1) + (self.matrix._31);
			let y = (self.matrix._12*points[i].0) + (self.matrix._22*points[i].1) + (self.matrix._32);
			trans[i].0 = x;
			trans[i].1 = y;
		}
    }
}