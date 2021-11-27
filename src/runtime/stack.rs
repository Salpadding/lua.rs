use core::panicking::panic;

use crate::runtime::LValue;

pub const LUAI_MAXSTACK: i32 = 1000000;
pub const LUA_REGISTRYINDEX: i32 = -LUAI_MAXSTACK - 1000;

pub struct LStack {
    slots: Vec<LValue>,
    len: usize,
}

impl LStack {
    pub fn new(size: usize) -> Self {
        Self {
            slots: vec![LValue::Nil; size],
            len: 0,
        }
    }

    pub fn check(&mut self, n: usize) {
        let mut free = self.slots.len() - self.len;

        while free < n {
            self.slots.push(LValue::Nil);
            free += 1
        }
    }

    pub fn push(&mut self, v: LValue) {
        self.slots[self.len] = v;
        self.len += 1;
    }

    pub fn pop(&mut self) -> LValue {
        let t = self.slots[self.len - 1].take();
        self.len -= 1;
        t
    }

    pub fn abs_idx(&self, i: i32) -> i32 {
        if i >= 0 {
            i
        } else {
            (self.len as i32 + 1 + i)
        }
    }

    pub fn is_valid(&self, i: i32) -> bool {
        let abs = self.abs_idx(i);
        abs > 0 && abs <= self.len as i32
    }

    pub fn get(&mut self, i: i32) -> LValue {
        let abs = self.abs_idx(i);

        if abs > 0 && abs <= self.len as i32 {
            return self.slots[(abs - 1) as usize].clone();
        }

        return LValue::Nil;
    }

    pub fn set(&mut self, i: i32, v: LValue) {
        let abs = self.abs_idx(i);
        if abs > 0 && abs <= self.len as i32 {
            self.slots[(abs - 1) as usize] = v;
            return;
        }

        panic!("invalid index {}", i);
    }
}