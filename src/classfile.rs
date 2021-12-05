use nom::{bytes::complete::tag, multi::count, number::complete::be_u16, IResult};

use crate::{
    attribute::{Attribute, AttributeInfo},
    constantpool::ConstantPool,
    fieldinfo::FieldInfo,
    methodinfo::MethodInfo,
};

#[derive(Debug)]
pub struct ClassFile {
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool_count: u16,
    pub constant_pool: Vec<ConstantPool>,
    pub access_flags: u16,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces_count: u16,
    pub interfaces: Vec<u16>,
    pub fields_count: u16,
    pub fields: Vec<FieldInfo>,
    pub methods_count: u16,
    pub methods: Vec<MethodInfo>,
    pub attributes_count: u16,
    pub attributes: Vec<Attribute>,
}

impl ClassFile {
    pub fn parse_class_file(buf: &[u8]) -> IResult<&[u8], ClassFile> {
        println!("{:X?}", buf);
        let (buf, _magic) = tag(0xCAFEBABEu32.to_be_bytes())(buf)?;
        let (buf, minor_version) = be_u16(buf)?;
        let (buf, major_version) = be_u16(buf)?;
        let (buf, constant_pool_count) = be_u16(buf)?;
        let (buf, constant_pool) = ConstantPool::parse(buf, constant_pool_count as usize)?;
        let (buf, access_flags) = be_u16(buf)?;
        let (buf, this_class) = be_u16(buf)?;
        let (buf, super_class) = be_u16(buf)?;
        let (buf, interfaces_count) = be_u16(buf)?;
        let (buf, interfaces) = count(be_u16, interfaces_count as usize)(buf)?;
        let (buf, fields_count) = be_u16(buf)?;
        let (buf, fields) = FieldInfo::parse(buf, fields_count, &constant_pool)?;
        let (buf, methods_count) = be_u16(buf)?;
        let (buf, methods) = MethodInfo::parse(buf, methods_count, &constant_pool)?;
        let (buf, attributes_count) = be_u16(buf)?;
        let (buf, attributes) = count(AttributeInfo::parse, attributes_count as usize)(buf)?;
        let attributes = Attribute::from_attribute_info(attributes, &constant_pool);

        Ok((
            buf,
            ClassFile {
                minor_version,
                major_version,
                constant_pool_count,
                constant_pool,
                access_flags,
                this_class,
                super_class,
                interfaces_count,
                interfaces,
                fields_count,
                fields,
                methods_count,
                methods,
                attributes_count,
                attributes,
            },
        ))
    }
}
