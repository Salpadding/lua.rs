use con::*;

use crate::cursor::FromCursor;

mod con {
    pub const MAGIC: u32 = 0x1B4C7561;
    pub const LUAC_DATA: [u8; 6] = [0x19, 0x93, 0x0d, 0x0a, 0x1a, 0x0a];
    pub const LUAC_INT: u64 = 0x5678;
}

#[derive(Default, Debug)]
pub struct Header {
    pub sig: u32,
    pub version: u8,
    pub format: u8,
    pub luac_data: [u8; 6],
    pub c_int_size: u8,
    pub size_t_size: u8,
    pub ins_size: u8,
    pub lua_int_size: u8,
    pub lua_num_size: u8,
    pub luac_int: u64,
    pub luac_num: f64,
}

impl FromCursor for Header {
    fn from_cursor(cur: &mut crate::cursor::Cursor) -> anyhow::Result<Self> {
        let mut h = Header::default();
        h.sig = cur.u32_be();
        asf!(h, sig, con::MAGIC);

        h.version = cur.u8();
        asf!(h, version, 0x53);

        h.format = cur.u8();
        asf!(h, format, 0);

        cur.read(&mut h.luac_data);
        asf!(h, luac_data, LUAC_DATA);

        h.c_int_size = cur.u8();
        asf!(h, c_int_size, 4);

        h.size_t_size = cur.u8();
        asf!(h, size_t_size, 8);

        h.ins_size = cur.u8();
        asf!(h, ins_size, 4);

        h.lua_int_size = cur.u8();
        asf!(h, lua_int_size, 8);

        h.lua_num_size = cur.u8();
        asf!(h, lua_num_size, 8);

        h.luac_int = cur.u64();
        if h.luac_int != LUAC_INT {
            return err!("invalid luac endian, expect little endian encoding of 0x5678, while {:X} found", h.luac_int);
        }

        h.luac_num = cur.f64();
        asf!(h, luac_num, 370.5);
        Ok(h)
    }
}
