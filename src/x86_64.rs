use crate::frame::Frame;
use std::arch::asm;
pub fn unwind() -> Vec<Frame> {
    let mut lr: usize;
    let mut fp: usize;
    unsafe {
        asm!("mov {}, rbp", out(reg) fp);
    }
    let mut frames = Vec::new();
    loop {
        let sp = fp + 0x10;
        let prev_fp = fp;
        (fp, lr) = load_prev_frame(fp);
        if fp == 0 || prev_fp == fp {
            break;
        }
        frames.push(Frame::new(sp, lr, fp));
    }
    frames
}
fn load_prev_frame(fp: usize) -> (usize, usize) {
    let mut prev_fp: usize;
    let mut prev_ip: usize;
    unsafe {
        asm!("mov {}, [{} + 0]", out(reg) prev_fp, in(reg) fp);
        asm!("mov {}, [{} + 8]", out(reg) prev_ip, in(reg) fp);
    }
    (prev_fp, prev_ip)
}
