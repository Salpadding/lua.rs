#![allow(warnings, unused)]
macro_rules! err {
    ($($arg:tt)*) => {{
        let res = format!($($arg)*);
        Result::Err(anyhow::Error::msg(res))
    }}
}
mod chunk;
mod cursor;
mod runtime;
mod ins;

pub type XRc<T> = std::rc::Rc<T>;

fn main() {
    println!("Hello, world!");
}
