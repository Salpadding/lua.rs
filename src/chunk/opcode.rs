use num_derive::FromPrimitive;
use crate::cursor::{Cursor, FromCursor};

#[derive(Debug, Clone, Copy)]
pub struct FatIns {
    pub op: OpCode,
    pub bits: u32,
}

impl FromCursor for FatIns {
    fn from_cursor(cur: &mut Cursor) -> anyhow::Result<Self> {
        let bits = cur.u32();
        let op = bits & 0x3f;
        let o: Option<OpCode> = num_traits::FromPrimitive::from_u32(op);

        if o.is_none() {
            return err!("unknown opcode {:X}", op);
        }
        Ok(Self {
            bits,
            op: o.unwrap(),
        })
    }
}

pub trait Ins {
    fn iABC(&self) -> (u16, u16, u16);

    fn iABx(&self) -> (u16, u32);

    fn iAsBx(&self) -> (u16, i32) {
        let (a, y) = self.iABx();
        let y = y as i32;
        (a, y - MAXARG_sBx as i32)
    }

    fn iAx(&self) -> u32;
}

const A: u32 = 0xff << 6;
const B: u32 = 0x1ff << (6 + 8);
const C: u32 = 0x1ff << (6 + 8 + 9);
const Bx: u32 = 0x3ffff << 14;
const Ax: u32 = 0x3ffffff << 6;
const MAXARG_sBx: u32 = (1 << 18 - 1) >> 1;

impl Ins for u32 {
    fn iABC(&self) -> (u16, u16, u16) {
        (
            ((self & A) >> 6) as u16,
            ((self & B) >> 14) as u16,
            ((self & C) >> 23) as u16
        )
    }

    fn iABx(&self) -> (u16, u32) {
        (
            ((self & A) >> 6) as u16,
            ((self & Bx) >> 14),
        )
    }

    fn iAx(&self) -> u32 {
        (self & Ax) >> 6
    }
}

/* OpCode */
#[repr(u8)]
#[derive(FromPrimitive, Debug, Clone, Copy, PartialEq)]
pub enum OpCode {
    OP_MOVE = 1,
    OP_LOADK,
    OP_LOADKX,
    OP_LOADBOOL,
    OP_LOADNIL,
    OP_GETUPVAL,
    OP_GETTABUP,
    OP_GETTABLE,
    OP_SETTABUP,
    OP_SETUPVAL,
    OP_SETTABLE,
    OP_NEWTABLE,
    OP_SELF,
    OP_ADD,
    OP_SUB,
    OP_MUL,
    OP_MOD,
    OP_POW,
    OP_DIV,
    OP_IDIV,
    OP_BAND,
    OP_BOR,
    OP_BXOR,
    OP_SHL,
    OP_SHR,
    OP_UNM,
    OP_BNOT,
    OP_NOT,
    OP_LEN,
    OP_CONCAT,
    OP_JMP,
    OP_EQ,
    OP_LT,
    OP_LE,
    OP_TEST,
    OP_TESTSET,
    OP_CALL,
    OP_TAILCALL,
    OP_RETURN,
    OP_FORLOOP,
    OP_FORPREP,
    OP_TFORCALL,
    OP_TFORLOOP,
    OP_SETLIST,
    OP_CLOSURE,
    OP_VARARG,
    OP_EXTRAARG,
}

