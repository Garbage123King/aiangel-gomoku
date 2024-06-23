#[derive(Copy, Clone)]
pub struct G7r {
    pub black: i8,
    pub white: i8,
    pub empty: i8,
}

pub fn create_g7r() -> G7r {
    G7r {
        black: 0,
        white: 0,
        empty: 0b1111111,
    }
}

fn g7r_has_no_white(a: &G7r) -> bool {
    a.white==0
}

// 测试模块
#[cfg(test)]

// 测试函数2：测试 add 函数的边界情况
#[test]
fn test_create_g7r() {
    let my_struct = create_g7r();
    assert_eq!(my_struct.black, 0);
    assert_eq!(my_struct.white, 0);
    assert_eq!(my_struct.empty, 0b1111111);
}

#[test]
fn test_g5r_has_no_white() {
    let mut my_struct = create_g7r();
    my_struct.black=0b1111111;
    my_struct.white=0b1111111;
    let result = g7r_has_no_white(&my_struct);
    assert_eq!(result, false);

    my_struct.white=0b0000000;
    let result = g7r_has_no_white(&my_struct);
    assert_eq!(result, true);
}

