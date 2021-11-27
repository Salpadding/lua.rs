macro_rules! asf {
    ($x: ident, $f: ident, $ex: expr) => {
        {
            if $x.$f != $ex {
                return err!("parse lua chunk failed: expect {:?} to be {:?} while {:?} found", stringify!($f), $ex, $x.$f);
            }
        }
    };
}

pub mod proto;
mod header;
pub mod opcode;

use std::io::Read;
use crate::cursor::{Cursor, FromCursor};
use anyhow::Result;
use crate::chunk::header::Header;
use crate::chunk::proto::ProtoType;
use std::sync::Arc;

#[derive(Debug)]
pub struct Chunk {
    pub header: Header,
    pub proto: Arc<ProtoType>,
}

impl FromCursor for Chunk {
    fn from_cursor(cur: &mut Cursor) -> Result<Self> {
        let header: Header = cur.read_as()?;
        cur.u8();
        Ok(
            Self {
                header,
                proto: cur.read_as()?,
            }
        )
    }
}

impl<T: FromCursor> FromCursor for Arc<T> {
    fn from_cursor(cur: &mut Cursor) -> Result<Self> {
        let r: T = cur.read_as()?;
        Ok(Arc::new(r))
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
