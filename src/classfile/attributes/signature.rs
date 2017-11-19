use std::io::Read;

use classfile::util;

pub type Signature = u16;

pub fn read(reader: &mut Read) -> Signature {
    /*let attribute_length = */util::read_u32(reader);

    util::read_u16(reader)
}
