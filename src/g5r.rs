use super::base::*;

#[derive(Copy, Clone)]
pub struct G5r {
    pub black: i8,
    pub white: i8,
    pub empty: i8,
}

pub fn create_g5r() -> G5r {
    G5r {
        black: 0,
        white: 0,
        empty: 0b11111,
    }
}

fn g5r_has_no_white(a: &G5r) -> bool {
    a.white==0
}

fn g5r_get_blocked(a: &G5r) -> bool {
    a.white != 0
}

fn g5r_black_win(a: &G5r) -> bool {
    a.white==0b11111
}

fn g5r_black_four(a: &G5r) -> bool {
    if g5r_get_blocked(a){
        false
    }else{
        a.black==0b11110 ||
        a.black==0b11101 ||
        a.black==0b11011 ||
        a.black==0b10111 ||
        a.black==0b01111
    }
}

fn g5r_fill_i8_by_index(a: &mut i8, i: i8){
    *a = match i {
        0 => *a | 0b10000,
        1 => *a | 0b01000,
        2 => *a | 0b00100,
        3 => *a | 0b00010,
        4 => *a | 0b00001,
        _ => panic!("Unexpected g5 index value: {}", i),
    };
}

fn g5r_fade_i8_by_index(a: &mut i8, i: i8){
    *a = match i {
        0 => *a & 0b01111,
        1 => *a & 0b10111,
        2 => *a & 0b11011,
        3 => *a & 0b11101,
        4 => *a & 0b11110,
        _ => panic!("Unexpected g5 index value: {}", i),
    };
}

fn g5r_put_by_index(put: &mut i8, emp: &mut i8, i:i8){
    g5r_fill_i8_by_index(put, i);
    g5r_fade_i8_by_index(emp, i);
}

fn g5r_put_by_index_color(a: &mut G5r, i:i8, color: Color){
    //#[cfg(debug_assertions)]

    if color == Color::Black
    {
        g5r_put_by_index(&mut a.black, &mut a.empty, i);
    }
    else
    {
        g5r_put_by_index(&mut a.white, &mut a.empty, i);
    }
}

// 测试模块
#[cfg(test)]

#[test]
fn test_create_g5r() {
    let my_struct = create_g5r();
    assert_eq!(my_struct.black, 0);
    assert_eq!(my_struct.white, 0);
    assert_eq!(my_struct.empty, 0b11111);
}

#[test]
fn test_g5r_has_no_white() {
    let mut my_struct = create_g5r();
    my_struct.black=0b00111;
    my_struct.white=0b11000;
    let result = g5r_has_no_white(&my_struct);
    assert_eq!(result, false);

    my_struct.white=0b00000;
    let result = g5r_has_no_white(&my_struct);
    assert_eq!(result, true);
}

#[test]
fn test_g5r_black_four() {
    let mut my_struct = create_g5r();
    my_struct.black=0b00111;
    my_struct.white=0b11000;
    let result = g5r_black_four(&my_struct);
    assert_eq!(result, false);

    my_struct.black=0b11011;
    my_struct.white=0b00100;
    let result = g5r_black_four(&my_struct);
    assert_eq!(result, false);

    my_struct.black=0b11111;
    my_struct.white=0b00000;
    let result = g5r_black_four(&my_struct);
    assert_eq!(result, false);

    my_struct.black=0b11011;
    my_struct.white=0b00000;
    let result = g5r_black_four(&my_struct);
    assert_eq!(result, true);
}

#[test]
fn test_g5r_fill_i8_by_index() {
    let mut a=0b11000;
    g5r_fill_i8_by_index(&mut a, 2);
    assert_eq!(a, 0b11100);
}

#[test]
fn test_g5r_fade_i8_by_index() {
    let mut a=0b11000;
    g5r_fade_i8_by_index(&mut a, 0);
    assert_eq!(a, 0b01000);
}

#[test]
fn test_g5r_put_by_index_color() {
    let mut my_struct = create_g5r();
    my_struct.black=0b00110;
    my_struct.white=0b01000;
    my_struct.empty=0b10001;

    g5r_put_by_index_color(&mut my_struct, 0, Color::Black);
    assert_eq!(my_struct.black, 0b10110);
    assert_eq!(my_struct.empty, 0b00001);

    g5r_put_by_index_color(&mut my_struct, 4, Color::White);
    assert_eq!(my_struct.white, 0b01001);
    assert_eq!(my_struct.empty, 0b00000);
}