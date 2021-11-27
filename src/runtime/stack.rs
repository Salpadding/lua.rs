use crate::runtime::LValue;

pub const  LUAI_MAXSTACK: i32 = 1000000;
pub const  LUA_REGISTRYINDEX: i32 = -LUAI_MAXSTACK - 1000;

pub struct LStack {
    slots: Vec<LValue>,
    top: usize,
}

impl LStack {
    pub fn new(size: usize) -> Self {
        Self {
            slots: vec![LValue::None; size],
            top: 0,
        }
    }
}