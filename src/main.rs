macro_rules! err {
    ($($arg:tt)*) => {{
        let res = format!($($arg)*);
        Result::Err(anyhow::Error::msg(res))
    }}
}
mod chunk;
mod cursor;

fn main() {
    println!("Hello, world!");
}
