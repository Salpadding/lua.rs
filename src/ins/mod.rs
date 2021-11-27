mod basic;

use crate::chunk::opcode::FatIns;
use crate::runtime::state::LState;

pub trait RTIns {
    fn execute(&self, state: &mut LState);
}

impl RTIns for FatIns {
    fn execute(&self, state: &mut LState) {
    }
}

trait BasicIns {
    fn get_up_value(&self, state: &mut LState);
}



