use crate::chunk::opcode::FatIns;
use crate::ins::BasicIns;
use crate::runtime::state::LState;

impl BasicIns for FatIns {
    fn get_up_value(&self, state: &mut LState) {

    }
}