pub struct Triangle {
    pub v1: i32,
    pub v2: i32,
    pub v3: i32,
}

impl Triangle {
    pub fn to_str(&self) -> String{
        format!("f {} {} {}\n", self.v1,self.v2,self.v3)
    }
}
