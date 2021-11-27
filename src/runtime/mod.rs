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
    String(String),
    Table,
    Function
}

impl Default for LValue {
    fn default() -> Self {
       Self::None
    }
}

