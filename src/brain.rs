
use super::g5r::*;
use super::g7r::*;
use super::base::*;

pub struct Brain {
    color: Color,  //Black or White

    //20×20
    g5r: [G5r; LEN_5],
    g7r: [G7r; LEN_7],

    //15×15
    //g5r_15: [G5r; cal_len!(15, 5)],
    //g7r_15: [G7r; cal_len!(15, 7)],

}

pub fn create_brain(color: Color) -> Brain {
    Brain {
        color: color,
        g5r: {
            let mut array = [create_g5r(); LEN_5];
            array
        },
        g7r: {
            let mut array = [create_g7r(); LEN_7];
            array
        },
        // g5r15: {
        //     let mut array = [create_g5r(); cal_len!(15, 5)];
        //     array
        // },
        // g7r15: {
        //     let mut array = [create_g7r(); cal_len!(15, 7)];
        //     array
        // },
    }
}

#[test]
fn test_create_brain() {
    let a = create_brain(Color::Black);
    assert_eq!(a.color, Color::Black);
    assert_eq!(a.g5r[0].black, 0);
    assert_eq!(a.g5r[0].white, 0);
    assert_eq!(a.g5r[0].empty, 0b11111);
    assert_eq!(a.g5r[137].black, 0);
    assert_eq!(a.g5r[137].white, 0);
    assert_eq!(a.g5r[137].empty, 0b11111);
    let b = create_brain(Color::White);
    assert_eq!(b.color, Color::White);
}
