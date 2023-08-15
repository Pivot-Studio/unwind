mod frame;

pub use frame::*;

#[cfg(target_arch = "aarch64")]
mod aarch64;
#[cfg(target_arch = "aarch64")]
pub use aarch64::*;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_unwind() {
        let frames1 = unwind();
        let mut frames2 = Vec::new();
        backtrace::trace(|frame| {
            frames2.push(Frame::new(frame.sp() as usize, frame.ip() as usize, 0));
            true
        });

        assert_eq!(frames1.len(), frames2.len() - 2);
        for i in 0..frames1.len() {
            assert_eq!(frames1[i].sp(), frames2[i + 2].sp());
            if i == 0 {
                continue;
            }
            assert_eq!(frames1[i].ip(), frames2[i + 2].ip());
        }
    }
}
