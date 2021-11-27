use crate::chunk::opcode::FatIns;
use crate::runtime::state::LState;

mod basic;

pub trait RTIns {
    fn execute(&self, state: &mut LState);
}

impl RTIns for FatIns {
    fn execute(&self, state: &mut LState) {}
}

trait BasicIns {
    fn mv(&self, state: &mut LState);
}



