use crate::XRc;

mod lvalue;
pub mod stack;
pub mod state;
mod vm;

#[derive(Clone, Debug)]
pub enum LValue {
    None,
    Nil,
    Bool(bool),
    F64(f64),
    I64(i64),
    String(XRc<String>),
    Table,
    Function,
}

impl LValue {
    pub fn take(&mut self) -> LValue {
        let mut r = LValue::Nil;
        std::mem::swap(&mut r, &mut self);
        r
    }
}

impl Default for LValue {
    fn default() -> Self {
        Self::None
    }
}

