use std::sync::Arc;

use anyhow::Result;

use crate::chunk::opcode::FatIns;
use crate::chunk::proto::LCons::Long;
use crate::cursor::{Cursor, FromCursor};
use crate::XRc;

pub mod cons_tag {
    pub const NIL: u8 = 0;
    pub const BOOL: u8 = 1;
    pub const NUMBER: u8 = 3;
    pub const INTEGER: u8 = 0x13;
    pub const SHORT_STR: u8 = 4;
    pub const LONG_STR: u8 = 0x14;
}

#[derive(Debug)]
pub struct LocalVar {
    pub name: String,
    pub start_pc: u32,
    pub end_pc: u32,
}

impl FromCursor for LocalVar {
    fn from_cursor(cur: &mut Cursor) -> Result<Self> {
        Ok(
            Self {
                name: cur.read_as()?,
                start_pc: cur.u32(),
                end_pc: cur.u32(),
            }
        )
    }
}

#[derive(Debug)]
pub enum LCons {
    String { tag: u8, data: XRc<String> },
    Long { tag: u8, data: u64 },
}

impl FromCursor for LCons {
    fn from_cursor(cur: &mut Cursor) -> Result<Self> {
        let tag = cur.u8();
        let w = match tag {
            cons_tag::NIL => Long { tag, data: 0 },
            cons_tag::BOOL => Long { tag, data: cur.u8() as u64 },
            cons_tag::INTEGER => Long { tag, data: cur.u64() },
            cons_tag::NUMBER => Long { tag, data: cur.u64() },
            cons_tag::SHORT_STR | cons_tag::LONG_STR => LCons::String { tag, data: cur.read_as()? },
            _ => return err!("invalid constant tag {:X}", tag),
        };
        Ok(w)
    }
}

#[derive(Debug, Default)]
pub struct UpValue {
    pub in_stack: u8,
    pub idx: u8,
}

impl FromCursor for UpValue {
    fn from_cursor(cur: &mut Cursor) -> Result<Self> {
        Ok(
            Self {
                in_stack: cur.u8(),
                idx: cur.u8(),
            }
        )
    }
}

#[derive(Debug, Default)]
pub struct ProtoType {
    pub source: String,
    pub line_defined: u32,
    pub last_line: u32,
    pub num_params: u8,
    pub var_arg: bool,
    pub max_stack: u8,
    pub code: Vec<FatIns>,
    pub cons: Vec<LCons>,
    pub up_values: Vec<UpValue>,
    pub protos: Vec<XRc<ProtoType>>,
    pub line_info: Vec<u32>,
    pub local_vars: Vec<u32>,
    pub up_value_names: Vec<String>,
}

impl FromCursor for ProtoType {
    fn from_cursor(cur: &mut Cursor) -> Result<Self> {
        let r: Self = Self {
            source: cur.read_as()?,
            line_defined: cur.u32(),
            last_line: cur.u32(),
            num_params: cur.u8(),
            var_arg: cur.u8() != 0,
            max_stack: cur.u8(),
            code: cur.read_vec()?,
            cons: cur.read_vec()?,
            up_values: cur.read_vec()?,
            protos: cur.read_vec()?,
            line_info: cur.read_vec()?,
            local_vars: cur.read_vec()?,
            up_value_names: cur.read_vec()?,
        };
        Ok(r)
    }
}