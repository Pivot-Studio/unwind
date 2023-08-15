#[derive(Debug)]
pub struct Frame {
    sp: usize,
    ip: usize,
    fp: usize,
}

impl Frame {
    pub fn sp(&self) -> usize {
        self.sp
    }
    pub fn ip(&self) -> usize {
        self.ip
    }
    pub fn new(sp: usize, ip: usize, fp: usize) -> Self {
        Self { sp, ip, fp }
    }
    pub fn fp(&self) -> usize {
        self.fp
    }
}
