
use super::gr5::*;
use super::gr7::*;

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

const LEN_5: usize = cal_len!(20, 5);
const LEN_7: usize = cal_len!(20, 7);

pub struct Brain {
    color: Color,  //Black or White

    //20×20
    gr5s: [Gr5; LEN_5],
    gr7s: [Gr7; LEN_7],

    //15×15
    //gr5s_15: [Gr5; cal_len!(15, 5)],
    //gr7s_15: [Gr7; cal_len!(15, 7)],

}

pub fn create_brain(color: Color) -> Brain {
    Brain {
        color: color,
        gr5s: {
            let mut array = [create_gr5(); cal_len!(20, 5)];
            array
        },
        gr7s: {
            let mut array = [create_gr7(); cal_len!(20, 7)];
            array
        },
        // gr515: {
        //     let mut array = [create_gr5(); cal_len!(15, 5)];
        //     array
        // },
        // gr715: {
        //     let mut array = [create_gr7(); cal_len!(15, 7)];
        //     array
        // },
    }
}

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

#[test]
fn test_create_brain() {
    let a = create_brain(Color::Black);
    assert_eq!(a.color, Color::Black);
    assert_eq!(a.gr5s[0].me, 0);
    assert_eq!(a.gr5s[0].opp, 0);
    assert_eq!(a.gr5s[0].emp, 0b11111);
    assert_eq!(a.gr5s[137].me, 0);
    assert_eq!(a.gr5s[137].opp, 0);
    assert_eq!(a.gr5s[137].emp, 0b11111);
    let b = create_brain(Color::White);
    assert_eq!(b.color, Color::White);
}
