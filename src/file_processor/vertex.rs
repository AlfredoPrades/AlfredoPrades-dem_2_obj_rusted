pub struct Vertex {
    pub x: i32,
    pub y: i32,
    pub z: f32,
}

impl Vertex {
    pub fn to_str(&self) -> String{
        format!("v {} {} {}\n", self.x,self.y,self.z)
    }
}
