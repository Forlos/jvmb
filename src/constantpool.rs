use nom::{
    multi::length_data,
    number::complete::{be_f32, be_f64, be_i32, be_i64, be_u16, u8},
    IResult,
};

const CONSTANT_CLASS: u8 = 7;
const CONSTANT_FIELD_REF: u8 = 9;
const CONSTANT_METHOD_REF: u8 = 10;
const CONSTANT_INTERFACE_METHOD_REF: u8 = 11;
const CONSTANT_STRING: u8 = 8;
const CONSTANT_INTEGER: u8 = 3;
const CONSTANT_FLOAT: u8 = 4;
const CONSTANT_LONG: u8 = 5;
const CONSTANT_DOUBLE: u8 = 6;
const CONSTANT_NAME_AND_TYPE: u8 = 12;
const CONSTANT_UTF8: u8 = 1;
const CONSTANT_METHOD_HANDLE: u8 = 15;
const CONSTANT_METHOD_TYPE: u8 = 16;
const CONSTANT_DYNAMIC: u8 = 17;
const CONSTANT_INVOKE_DYNAMIC: u8 = 18;
const CONSTANT_MODULE: u8 = 19;
const CONSTANT_PACKAGE: u8 = 20;

#[derive(Debug)]
pub enum ConstantPool {
    Class(u16),
    FieldRef(u16, u16),
    MethodRef(u16, u16),
    InterfaceMethodRef(u16, u16),
    String(u16),
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    NameAndType(u16, u16),
    UTF8(String),
    MethodHandle(u8, u16),
    MethodType(u16),
    Dynamic(u16, u16),
    InvokeDynamic(u16, u16),
    Module(u16),
    Package(u16),
}

impl ConstantPool {
    pub fn parse(mut buf: &[u8], constant_pool_count: usize) -> IResult<&[u8], Vec<ConstantPool>> {
        let mut constant_pool = Vec::with_capacity(constant_pool_count - 1);

        let mut i = 0;
        while i < constant_pool_count - 1 {
            let (temp_buf, constant) = Self::parse_constant(buf)?;
            buf = temp_buf;
            if let ConstantPool::Long(_) = constant {
                i += 1;
            } else if let ConstantPool::Float(_) = constant {
                i += 1;
            }
            constant_pool.push(constant);

            i += 1;
        }

        Ok((buf, constant_pool))
    }

    fn parse_constant(buf: &[u8]) -> IResult<&[u8], ConstantPool> {
        let (buf, tag) = u8(buf)?;
        match tag {
            CONSTANT_CLASS => {
                let (buf, name_index) = be_u16(buf)?;
                Ok((buf, ConstantPool::Class(name_index)))
            }
            CONSTANT_FIELD_REF => {
                let (buf, class_index) = be_u16(buf)?;
                let (buf, name_and_type_index) = be_u16(buf)?;
                Ok((
                    buf,
                    ConstantPool::FieldRef(class_index, name_and_type_index),
                ))
            }
            CONSTANT_METHOD_REF => {
                let (buf, class_index) = be_u16(buf)?;
                let (buf, name_and_type_index) = be_u16(buf)?;
                Ok((
                    buf,
                    ConstantPool::MethodRef(class_index, name_and_type_index),
                ))
            }
            CONSTANT_INTERFACE_METHOD_REF => {
                let (buf, class_index) = be_u16(buf)?;
                let (buf, name_and_type_index) = be_u16(buf)?;
                Ok((
                    buf,
                    ConstantPool::InterfaceMethodRef(class_index, name_and_type_index),
                ))
            }
            CONSTANT_STRING => {
                let (buf, string_index) = be_u16(buf)?;
                Ok((buf, ConstantPool::String(string_index)))
            }
            CONSTANT_INTEGER => {
                let (buf, value) = be_i32(buf)?;
                Ok((buf, ConstantPool::Integer(value)))
            }
            CONSTANT_FLOAT => {
                let (buf, value) = be_f32(buf)?;
                Ok((buf, ConstantPool::Float(value)))
            }
            CONSTANT_LONG => {
                let (buf, value) = be_i64(buf)?;
                Ok((buf, ConstantPool::Long(value)))
            }
            CONSTANT_DOUBLE => {
                let (buf, value) = be_f64(buf)?;
                Ok((buf, ConstantPool::Double(value)))
            }
            CONSTANT_NAME_AND_TYPE => {
                let (buf, name_index) = be_u16(buf)?;
                let (buf, descriptor_index) = be_u16(buf)?;
                Ok((buf, ConstantPool::NameAndType(name_index, descriptor_index)))
            }
            CONSTANT_UTF8 => {
                let (buf, value) = length_data(be_u16)(buf)?;
                Ok((
                    buf,
                    ConstantPool::UTF8(String::from_utf8(value.to_vec()).unwrap()),
                ))
            }
            CONSTANT_METHOD_HANDLE => {
                let (buf, reference_kind) = u8(buf)?;
                let (buf, reference_index) = be_u16(buf)?;
                Ok((
                    buf,
                    ConstantPool::MethodHandle(reference_kind, reference_index),
                ))
            }
            CONSTANT_METHOD_TYPE => {
                let (buf, descriptor_index) = be_u16(buf)?;
                Ok((buf, ConstantPool::MethodType(descriptor_index)))
            }
            CONSTANT_DYNAMIC => {
                let (buf, bootstrap_method_attr_index) = be_u16(buf)?;
                let (buf, name_and_type_index) = be_u16(buf)?;
                Ok((
                    buf,
                    ConstantPool::Dynamic(bootstrap_method_attr_index, name_and_type_index),
                ))
            }
            CONSTANT_INVOKE_DYNAMIC => {
                let (buf, bootstrap_method_attr_index) = be_u16(buf)?;
                let (buf, name_and_type_index) = be_u16(buf)?;
                Ok((
                    buf,
                    ConstantPool::InvokeDynamic(bootstrap_method_attr_index, name_and_type_index),
                ))
            }
            CONSTANT_MODULE => {
                let (buf, name_index) = be_u16(buf)?;
                Ok((buf, ConstantPool::Module(name_index)))
            }
            CONSTANT_PACKAGE => {
                let (buf, name_index) = be_u16(buf)?;
                Ok((buf, ConstantPool::Package(name_index)))
            }
            _ => unimplemented!(),
        }
    }
}
