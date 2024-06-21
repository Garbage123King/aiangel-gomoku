#[derive(Copy, Clone)]
pub struct Gr5 {
    pub me: i8,
    pub opp: i8,
    pub emp: i8,
}

pub fn create_gr5() -> Gr5 {
    Gr5 {
        me: 0,
        opp: 0,
        emp: 0b11111,
    }
}

fn gr5_has_no_opp(a: &Gr5) -> bool {
    a.opp==0
}

fn gr5_get_blocked(a: &Gr5) -> bool {
    a.opp != 0
}

fn gr5_me_win(a: &Gr5) -> bool {
    a.opp==0b11111
}

fn gr5_me_four(a: &Gr5) -> bool {
    if gr5_get_blocked(a){
        false
    }else{
        a.me==0b11110 ||
        a.me==0b11101 ||
        a.me==0b11011 ||
        a.me==0b10111 ||
        a.me==0b01111
    }
}

// 测试模块
#[cfg(test)]

#[test]
fn test_create_gr5() {
    let my_struct = create_gr5();
    assert_eq!(my_struct.me, 0);
    assert_eq!(my_struct.opp, 0);
    assert_eq!(my_struct.emp, 0b11111);
}

#[test]
fn test_gr5_has_no_opp() {
    let mut my_struct = create_gr5();
    my_struct.me=0b00111;
    my_struct.opp=0b11000;
    let result = gr5_has_no_opp(&my_struct);
    assert_eq!(result, false);

    my_struct.opp=0b00000;
    let result = gr5_has_no_opp(&my_struct);
    assert_eq!(result, true);
}

#[test]
fn test_gr5_me_four() {
    let mut my_struct = create_gr5();
    my_struct.me=0b00111;
    my_struct.opp=0b11000;
    let result = gr5_me_four(&my_struct);
    assert_eq!(result, false);

    my_struct.me=0b11011;
    my_struct.opp=0b00100;
    let result = gr5_me_four(&my_struct);
    assert_eq!(result, false);

    my_struct.me=0b11111;
    my_struct.opp=0b00000;
    let result = gr5_me_four(&my_struct);
    assert_eq!(result, false);

    my_struct.me=0b11011;
    my_struct.opp=0b00000;
    let result = gr5_me_four(&my_struct);
    assert_eq!(result, true);
}
