use crate::chunk::Chunk;
use std::sync::Arc;
use crate::runtime::state::LState;
use crate::chunk::opcode::OpCode;
use crate::chunk::proto::ProtoType;
use crate::ins::RTIns;

pub struct LuaVm {
    main: Arc<ProtoType>,
    state: LState,
}


impl LuaVm {
    pub fn new(main: Arc<ProtoType>) -> Self {
        Self {
            main: main.clone(),
            state: LState::new(main.max_stack as usize, main),
        }
    }

    pub fn run(&mut self) {
        loop {
            let ins = self.state.fetch();

            if ins.op == OpCode::OP_RETURN {
                break;
            }

            ins.execute(&mut self.state);
        }
    }
}