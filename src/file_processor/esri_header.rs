use std::fmt::{Formatter, Error, self};

pub struct EsriHeader {
    pub ncols: i32,
    pub nrows: i32,
    pub xllcenter: i32,
    pub yllcenter: i32,
    pub cellsize:i32,
    pub nodatavalue: f32,

}
impl Default for EsriHeader {
    fn default () -> EsriHeader {
        EsriHeader{ncols: 0, nrows: 0, xllcenter:0,yllcenter: 0,cellsize: 0,nodatavalue: 0.0}
    }
}

impl fmt::Display for EsriHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Header:\n ncols:{}\n nrows:{}\n xllcenter:{}\n yllcenter:{}\n cellsize:{}\n nodata_value:{}\n)", self.ncols, self.nrows,self.xllcenter,self.yllcenter,self.cellsize,self.nodatavalue)
    }
}


pub trait Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>;
}