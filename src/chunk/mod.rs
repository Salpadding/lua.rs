macro_rules! asf {
    ($x: ident, $f: ident, $ex: expr) => {
        {
            if $x.$f != $ex {
                return err!("parse lua chunk failed: expect {:?} to be {:?} while {:?} found", stringify!($f), $ex, $x.$f);
            }
        }
    };
}

mod proto;
mod header;
use std::io::Read;
use crate::cursor::{Cursor, FromCursor};
use anyhow::Result;
use crate::chunk::header::Header;
use crate::chunk::proto::Prototype;

#[derive(Debug)]
pub struct Chunk {
    pub header: Header,
    pub proto: Prototype,
}

impl FromCursor for Chunk {
    fn from_cursor(cur: &mut Cursor) -> Result<Self> {
        let header: Header =  cur.read_as()?;
        cur.u8();
        let proto: Prototype = cur.read_as()?;
        Ok(
            Self {
                header, proto
            }
        )
    }
}

#[cfg(test)]
mod test {
    use std::io::Read;
    use crate::chunk::Chunk;
    use crate::chunk::header::Header;
    use crate::cursor::Cursor;
    use crate::cursor::FromCursor;

    #[test]
    fn test() {
        let mut f = std::fs::File::open("testdata/ch2.bin").unwrap();
        let mut v: Vec<u8> = Vec::new();
        f.read_to_end(&mut v).unwrap();
        let mut cur = Cursor::new(&v);
        let hd: Chunk = cur.read_as().unwrap();

        println!("{:#?}", hd);
    }
}