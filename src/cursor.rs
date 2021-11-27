use anyhow::Result;

pub struct Cursor<'a> {
    data: &'a [u8],
    off: usize,
}

macro_rules! un {
    ($f: ident, $w: expr) => {
        pub fn $f(&mut self) -> $f {
            let mut cache: [u8; $w] = Default::default();
            cache.copy_from_slice(&self.data[self.off..self.off + $w]);
            self.off += $w;
            $f::from_le_bytes(cache)
        }
    };
}

macro_rules! un_be {
    ($f: ident, $t: ident, $w: expr) => {
        pub fn $f(&mut self) -> $t {
            let mut cache: [u8; $w] = Default::default();
            cache.copy_from_slice(&self.data[self.off..self.off + $w]);
            self.off += $w;
            $t::from_be_bytes(cache)
        }
    };
}

impl<'a> Cursor<'a> {
    pub fn new(slice: &'a [u8]) -> Self {
        Self {
            data: slice,
            off: 0,
        }
    }
    pub fn u8(&mut self) -> u8 {
        let r = self.data[self.off];
        self.off += 1;
        r
    }

    pub fn read(&mut self, buf: &mut [u8]) {
        buf.copy_from_slice(&self.data[self.off..self.off + buf.len()]);
        self.off += buf.len();
    }

    un!(u16, 2);
    un!(u32, 4);
    un_be!(u32_be, u32, 4);
    un!(u64, 8);
    un!(f64, 8);
    un!(i64, 8);

    pub fn read_as<T: FromCursor>(&mut self) -> Result<T> {
        T::from_cursor(self)
    }

    pub fn read_vec<T: FromCursor>(&mut self) -> Result<Vec<T>> {
        let n = self.u32() as usize;
        let mut v: Vec<T> = Vec::with_capacity(n);

        for _ in 0..n {
            v.push(self.read_as()?);
        }

        Ok(v)
    }
}

pub trait FromCursor: Sized {
    fn from_cursor(cur: &mut Cursor) -> Result<Self>;
}

impl FromCursor for u32 {
    fn from_cursor(cur: &mut Cursor) -> Result<Self> {
        Ok(cur.u32())
    }
}

impl FromCursor for String {
    fn from_cursor(cur: &mut Cursor) -> Result<Self> {
        let mut sz: usize = cur.u8() as usize;
        if sz == 0 {
            return Ok("".into());
        }

        if sz == 0xff {
            sz = cur.u64() as usize;
        }

        if sz == 0 || sz > cur.data.len() {
            return err!("parse chunk failed: invalid string size {}", sz);
        }
        let mut v: Vec<u8> = vec![0u8; sz - 1];
        cur.read(&mut v);

        Ok(
            String::from_utf8(v)?
        )
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {}
}