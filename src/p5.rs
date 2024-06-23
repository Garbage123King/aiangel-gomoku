use super::g5::*;
use super::base::*;

//横竖撇捺
#[derive(Copy, Clone)]
pub struct P5<'a> {
    pub id: u16,
    pub x: i8,
    pub y: i8,
    pub p: [&'a G5<'a>; LEN_5],
}