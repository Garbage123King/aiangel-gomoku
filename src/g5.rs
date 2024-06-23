use super::p5::*;
use super::base::*;

#[derive(Copy, Clone)]
pub struct G5<'a> {
    pub id: u16,
    pub p: [&'a P5<'a>; LEN],
}