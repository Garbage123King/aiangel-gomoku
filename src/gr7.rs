#[derive(Copy, Clone)]
pub struct Gr7 {
    pub me: i8,
    pub opp: i8,
    pub emp: i8,
}

pub fn create_gr7() -> Gr7 {
    Gr7 {
        me: 0,
        opp: 0,
        emp: 0b1111111,
    }
}

fn gr7_has_no_opp(a: &Gr7) -> bool {
    a.opp==0
}

// 测试模块
#[cfg(test)]

// 测试函数2：测试 add 函数的边界情况
#[test]
fn test_create_gr5() {
    let my_struct = create_gr7();
    assert_eq!(my_struct.me, 0);
    assert_eq!(my_struct.opp, 0);
    assert_eq!(my_struct.emp, 0b1111111);
}

#[test]
fn test_gr5_has_no_opp() {
    let mut my_struct = create_gr7();
    my_struct.me=0b1111111;
    my_struct.opp=0b1111111;
    let result = gr7_has_no_opp(&my_struct);
    assert_eq!(result, false);

    my_struct.opp=0b0000000;
    let result = gr7_has_no_opp(&my_struct);
    assert_eq!(result, true);
}

