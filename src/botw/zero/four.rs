use super::Control0;
use crate::{
    botw::{Control, RawControl, SubControl},
    Result,
};

use byteordered::Endian;

use msbt::Header;

use failure::ResultExt;

use serde_derive::{Deserialize, Serialize};

use std::io::{Cursor, Write};

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Control0_4 {
    field_1: u16,
}

impl SubControl for Control0_4 {
    fn marker(&self) -> u16 {
        4
    }

    fn parse(header: &Header, mut reader: &mut Cursor<&[u8]>) -> Result<Control> {
        Ok(Control::Raw(RawControl::Zero(Control0::Four(Control0_4 {
            field_1: header
                .endianness()
                .read_u16(&mut reader)
                .with_context(|_| "could not read field_1")?,
        }))))
    }

    fn write(&self, header: &Header, mut writer: &mut dyn Write) -> Result<()> {
        header
            .endianness()
            .write_u16(&mut writer, self.field_1)
            .with_context(|_| "could not write field 1")?;

        Ok(())
    }
}
