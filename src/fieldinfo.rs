use nom::{multi::count, number::complete::be_u16, IResult};

use crate::{
    attribute::{Attribute, AttributeInfo},
    constantpool::ConstantPool,
};

#[derive(Debug)]
pub struct FieldInfo {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes_count: u16,
    attributes: Vec<Attribute>,
}

impl FieldInfo {
    pub fn parse<'a>(
        mut buf: &'a [u8],
        fields_count: u16,
        constant_pool: &[ConstantPool],
    ) -> IResult<&'a [u8], Vec<FieldInfo>> {
        let mut fields = Vec::with_capacity(fields_count as usize);
        for _ in 0..fields_count {
            let (temp_buf, constant) = Self::parse_field_info(buf, constant_pool)?;
            buf = temp_buf;
            fields.push(constant);
        }

        Ok((buf, fields))
    }
    fn parse_field_info<'a>(
        buf: &'a [u8],
        constant_pool: &[ConstantPool],
    ) -> IResult<&'a [u8], FieldInfo> {
        let (buf, access_flags) = be_u16(buf)?;
        let (buf, name_index) = be_u16(buf)?;
        let (buf, descriptor_index) = be_u16(buf)?;
        let (buf, attributes_count) = be_u16(buf)?;
        let (buf, attributes) = count(AttributeInfo::parse, attributes_count as usize)(buf)?;
        let attributes = Attribute::from_attribute_info(attributes, constant_pool);
        Ok((
            buf,
            FieldInfo {
                access_flags,
                name_index,
                descriptor_index,
                attributes_count,
                attributes,
            },
        ))
    }
}
