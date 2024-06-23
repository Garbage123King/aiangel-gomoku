#[derive(Debug, PartialEq)] 
pub enum Color {
    Black,
    White,
}

macro_rules! cal_len {
    ($size:expr, $gl:expr) => {
        $size * ($size - $gl + 1) * 2 + 
        ((1 + $size - $gl + 1 - 1) * ($size - $gl) / 2 * 2 + ($size - $gl + 1)) *2
    };
}

pub const LEN:   usize = 20;
pub const LEN_5: usize = cal_len!(LEN, 5);
pub const LEN_7: usize = cal_len!(LEN, 7);


#[test]
fn test_cal_len() {
    let a = cal_len!(20, 5);
    assert_eq!(a, 1152);
    let b = cal_len!(15, 5);
    assert_eq!(b, 572);
    let c = cal_len!(10, 5);
    assert_eq!(c, 192);
    let d = cal_len!(6, 5);
    assert_eq!(d, 32);
}
