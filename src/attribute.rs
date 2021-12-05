use nom::{
    bytes::complete::take,
    multi::count,
    number::complete::{be_u16, be_u32, u8},
    IResult,
};

use crate::constantpool::ConstantPool;

#[derive(Debug)]
pub struct AttributeInfo {
    attribute_name_index: u16,
    attribute_length: u32,
    info: Vec<u8>,
}

impl AttributeInfo {
    pub fn parse(buf: &[u8]) -> IResult<&[u8], AttributeInfo> {
        let (buf, attribute_name_index) = be_u16(buf)?;
        let (buf, attribute_length) = be_u32(buf)?;
        let (buf, info) = take(attribute_length as usize)(buf)?;

        Ok((
            buf,
            AttributeInfo {
                attribute_name_index,
                attribute_length,
                info: info.to_vec(),
            },
        ))
    }
}

#[derive(Debug)]
pub enum Attribute {
    ConstantValue(u16),
    Code(Code),
    StackMapTable(StackMapTable),
    Exceptions(Exceptions),
    InnerClasses(InnerClasses),
    EnclosingMethod(EnclosingMethod),
    Synthetic,
    Signature(Signature),
    SourceFile(SourceFile),
    SourceDebugExtension(String),
    LineNumberTable(Vec<LineNumber>),
    LocalVariableTable(Vec<LocalVariable>),
    LocalVariableTypeTable(Vec<LocalVariableType>),
    Deprecated,
    RuntimeVisibleAnnotations(Vec<Annotation>),
    RuntimeInvisibleAnnotations(Vec<Annotation>),
    RuntimeVisibleParameterAnnotations(Vec<ParameterAnnotation>),
    RuntimeInvisibleParameterAnnotations(Vec<ParameterAnnotation>),
    RuntimeVisibleTypeAnnotations(Vec<TypeAnnotation>),
    RuntimeInvisibleTypeAnnotations(Vec<TypeAnnotation>),
    AnnotationDefault(ElementValue),
    BootstrapMethods(Vec<BootstrapMethod>),
    MethodParameters(Vec<Parameter>),
    Module(Module),
    ModulePackages(Vec<u16>),
    ModuleMainClass(u16),
    NestHost(u16),
    NestMembers(Vec<u16>),
    Record(Vec<RecordComponentInfo>),
    PermittedSubclasses(Vec<u16>),
}

impl Attribute {
    pub fn from_attribute_info(
        attributes: Vec<AttributeInfo>,
        constant_pool: &[ConstantPool],
    ) -> Vec<Attribute> {
        attributes
            .into_iter()
            .map(|attr| {
                Attribute::parse(
                    attr.attribute_name_index as usize,
                    &attr.info,
                    constant_pool,
                )
                .unwrap()
            })
            .collect()
    }
    fn parse<'a>(
        attribute_name_index: usize,
        info: &'a [u8],
        constant_pool: &[ConstantPool],
    ) -> Result<Self, nom::Err<nom::error::Error<&'a [u8]>>> {
        let attribute_type = constant_pool
            .get(attribute_name_index as usize - 1)
            .unwrap();

        if let ConstantPool::UTF8(name) = attribute_type {
            match name.as_str() {
                "ConstantValue" => {
                    let (_, constantvalue_index) = be_u16(info)?;
                    Ok(Attribute::ConstantValue(constantvalue_index))
                }
                "Code" => {
                    let (_, code) = Code::parse(info, constant_pool)?;
                    Ok(Attribute::Code(code))
                }
                "StackMapTable" => {
                    let (_, stack_map_table) = StackMapTable::parse(info)?;
                    Ok(Attribute::StackMapTable(stack_map_table))
                }
                "Exceptions" => {
                    let (_, exceptions) = Exceptions::parse(info)?;
                    Ok(Attribute::Exceptions(exceptions))
                }
                "InnerClasses" => {
                    let (_, inner_classes) = InnerClasses::parse(info)?;
                    Ok(Attribute::InnerClasses(inner_classes))
                }
                "EnclosingMethod" => {
                    let (_, enclosing_method) = EnclosingMethod::parse(info)?;
                    Ok(Attribute::EnclosingMethod(enclosing_method))
                }
                "Synthetic" => Ok(Attribute::Synthetic),
                "Signature" => {
                    let (_, signature) = Signature::parse(info)?;
                    Ok(Attribute::Signature(signature))
                }
                "SourceFile" => {
                    let (_, source_file) = SourceFile::parse(info)?;
                    Ok(Attribute::SourceFile(source_file))
                }
                "SourceDebugExtension" => Ok(Attribute::SourceDebugExtension(
                    String::from_utf8(info.to_vec()).unwrap(),
                )),
                "LineNumberTable" => {
                    let (buf, line_number_table_length) = be_u16(info)?;
                    let (_, line_number_table) =
                        count(LineNumber::parse, line_number_table_length as usize)(buf)?;
                    Ok(Attribute::LineNumberTable(line_number_table))
                }
                "LocalVariableTable" => {
                    let (buf, local_variable_table_length) = be_u16(info)?;
                    let (_, local_variable_table) =
                        count(LocalVariable::parse, local_variable_table_length as usize)(buf)?;
                    Ok(Attribute::LocalVariableTable(local_variable_table))
                }
                "LocalVariableTypeTable" => {
                    let (buf, local_variable_type_table_length) = be_u16(info)?;
                    let (_, local_variable_type_table) = count(
                        LocalVariableType::parse,
                        local_variable_type_table_length as usize,
                    )(buf)?;
                    Ok(Attribute::LocalVariableTypeTable(local_variable_type_table))
                }
                "Deprecated" => Ok(Attribute::Deprecated),
                "RuntimeVisibleAnnotations" => {
                    let (buf, num_annotations) = be_u16(info)?;
                    let (_, annotations) = count(Annotation::parse, num_annotations as usize)(buf)?;
                    Ok(Attribute::RuntimeVisibleAnnotations(annotations))
                }
                "RuntimeInvisibleAnnotations" => {
                    let (buf, num_annotations) = be_u16(info)?;
                    let (_, annotations) = count(Annotation::parse, num_annotations as usize)(buf)?;
                    Ok(Attribute::RuntimeInvisibleAnnotations(annotations))
                }
                "RuntimeVisibleParameterAnnotations" => {
                    let (buf, num_parameters) = u8(info)?;
                    let (_, parameter_annotations) =
                        count(ParameterAnnotation::parse, num_parameters as usize)(buf)?;
                    Ok(Attribute::RuntimeVisibleParameterAnnotations(
                        parameter_annotations,
                    ))
                }
                "RuntimeInvisibleParameterAnnotations" => {
                    let (buf, num_parameters) = u8(info)?;
                    let (_, parameter_annotations) =
                        count(ParameterAnnotation::parse, num_parameters as usize)(buf)?;
                    Ok(Attribute::RuntimeInvisibleParameterAnnotations(
                        parameter_annotations,
                    ))
                }
                "RuntimeVisibleTypeAnnotations" => {
                    let (buf, num_annotations) = be_u16(info)?;
                    let (_, annotations) =
                        count(TypeAnnotation::parse, num_annotations as usize)(buf)?;
                    Ok(Attribute::RuntimeVisibleTypeAnnotations(annotations))
                }
                "RuntimeInvisibleTypeAnnotations" => {
                    let (buf, num_annotations) = be_u16(info)?;
                    let (_, annotations) =
                        count(TypeAnnotation::parse, num_annotations as usize)(buf)?;
                    Ok(Attribute::RuntimeInvisibleTypeAnnotations(annotations))
                }
                "AnnotationDefault" => {
                    let (_, default_value) = ElementValue::parse(info)?;
                    Ok(Attribute::AnnotationDefault(default_value))
                }
                "BootstrapMethods" => {
                    let (buf, num_bootstrap_methods) = be_u16(info)?;
                    let (_, bootstrap_methods) =
                        count(BootstrapMethod::parse, num_bootstrap_methods as usize)(buf)?;
                    Ok(Attribute::BootstrapMethods(bootstrap_methods))
                }
                "MethodParameters" => {
                    let (buf, parameters_count) = u8(info)?;
                    let (_, parameters) = count(Parameter::parse, parameters_count as usize)(buf)?;
                    Ok(Attribute::MethodParameters(parameters))
                }
                "Module" => {
                    let (_, module) = Module::parse(info)?;
                    Ok(Attribute::Module(module))
                }
                "ModulePackages" => {
                    let (buf, package_count) = be_u16(info)?;
                    let (_, package_index) = count(be_u16, package_count as usize)(buf)?;
                    Ok(Attribute::ModulePackages(package_index))
                }
                "ModuleMainClass" => {
                    let (_, main_class_index) = be_u16(info)?;
                    Ok(Attribute::ModuleMainClass(main_class_index))
                }
                "NestHost" => {
                    let (_, host_class_index) = be_u16(info)?;
                    Ok(Attribute::NestHost(host_class_index))
                }
                "NestMembers" => {
                    let (buf, number_of_classes) = be_u16(info)?;
                    let (_, classes) = count(be_u16, number_of_classes as usize)(buf)?;
                    Ok(Attribute::NestMembers(classes))
                }
                "RecordComponentInfo" => {
                    let (mut buf, components_count) = be_u16(info)?;
                    let mut components = Vec::with_capacity(components_count as usize);
                    for _ in 0..components_count {
                        let (temp_buf, component) = RecordComponentInfo::parse(buf, constant_pool)?;
                        buf = temp_buf;
                        components.push(component);
                    }
                    Ok(Attribute::Record(components))
                }
                "PermittedSubclasses" => {
                    let (buf, number_of_classes) = be_u16(info)?;
                    let (_, classes) = count(be_u16, number_of_classes as usize)(buf)?;
                    Ok(Attribute::PermittedSubclasses(classes))
                }
                attr_type => unimplemented!("Unimplemented attribute type: {}", attr_type),
            }
        } else {
            unimplemented!()
        }
    }
}

#[derive(Debug)]
pub struct Code {
    pub max_stack: u16,
    pub max_locals: u16,
    pub code_length: u32,
    pub code: Vec<u8>,
    pub exception_table_length: u16,
    pub exception_table: Vec<Exception>,
    pub attributes_count: u16,
    pub attributes: Vec<Attribute>,
}

impl Code {
    fn parse<'a>(buf: &'a [u8], constant_pool: &[ConstantPool]) -> IResult<&'a [u8], Self> {
        let (buf, max_stack) = be_u16(buf)?;
        let (buf, max_locals) = be_u16(buf)?;
        let (buf, code_length) = be_u32(buf)?;
        let (buf, code) = take(code_length as usize)(buf)?;
        let (buf, exception_table_length) = be_u16(buf)?;
        let (buf, exception_table) = count(Exception::parse, exception_table_length as usize)(buf)?;
        let (buf, attributes_count) = be_u16(buf)?;
        let (buf, attributes) = count(AttributeInfo::parse, attributes_count as usize)(buf)?;
        let attributes = Attribute::from_attribute_info(attributes, constant_pool);

        Ok((
            buf,
            Code {
                max_stack,
                max_locals,
                code_length,
                code: code.to_vec(),
                exception_table_length,
                exception_table,
                attributes_count,
                attributes,
            },
        ))
    }
}

#[derive(Debug)]
pub struct Exception {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: u16,
}

impl Exception {
    fn parse(buf: &[u8]) -> IResult<&[u8], Self> {
        let (buf, start_pc) = be_u16(buf)?;
        let (buf, end_pc) = be_u16(buf)?;
        let (buf, handler_pc) = be_u16(buf)?;
        let (buf, catch_type) = be_u16(buf)?;

        Ok((
            buf,
            Self {
                start_pc,
                end_pc,
                handler_pc,
                catch_type,
            },
        ))
    }
}

#[derive(Debug)]
pub struct StackMapTable {
    pub entries: Vec<StackMapFrame>,
}

impl StackMapTable {
    fn parse<'a>(buf: &'a [u8]) -> IResult<&'a [u8], Self> {
        let (buf, number_of_entries) = be_u16(buf)?;
        let (buf, entries) = count(StackMapFrame::parse, number_of_entries as usize)(buf)?;

        Ok((buf, StackMapTable { entries }))
    }
}

#[derive(Debug)]
pub enum StackMapFrame {
    SameFrame,
    SameLocals1StackItemFrame(VerificationTypeInfo),
    SameLocals1StackItemFrameExtended(u16, VerificationTypeInfo),
    ChopFrame(u16),
    SameFrameExtended(u16),
    AppendFrame(u16, Vec<VerificationTypeInfo>),
    FullFrame(
        u16,
        u16,
        Vec<VerificationTypeInfo>,
        u16,
        Vec<VerificationTypeInfo>,
    ),
}

impl StackMapFrame {
    fn parse<'a>(buf: &'a [u8]) -> IResult<&'a [u8], Self> {
        let (buf, frame_type) = u8(buf)?;
        match frame_type {
            0..=63 => Ok((buf, StackMapFrame::SameFrame)),
            64..=127 => {
                let (buf, verification_type_info) = VerificationTypeInfo::parse(buf)?;
                Ok((
                    buf,
                    StackMapFrame::SameLocals1StackItemFrame(verification_type_info),
                ))
            }
            247 => {
                let (buf, offset_delta) = be_u16(buf)?;
                let (buf, verification_type_info) = VerificationTypeInfo::parse(buf)?;
                Ok((
                    buf,
                    StackMapFrame::SameLocals1StackItemFrameExtended(
                        offset_delta,
                        verification_type_info,
                    ),
                ))
            }
            248..=250 => {
                let (buf, offset_delta) = be_u16(buf)?;
                Ok((buf, StackMapFrame::ChopFrame(offset_delta)))
            }
            251 => {
                let (buf, offset_delta) = be_u16(buf)?;
                Ok((buf, StackMapFrame::SameFrameExtended(offset_delta)))
            }
            x if (242..=254).contains(&x) => {
                dbg!(x);
                let (buf, offset_delta) = be_u16(buf)?;
                dbg!(buf, offset_delta);
                let (buf, locals) = count(VerificationTypeInfo::parse, x as usize - 251)(buf)?;
                Ok((buf, StackMapFrame::AppendFrame(offset_delta, locals)))
            }
            255 => {
                let (buf, offset_delta) = be_u16(buf)?;
                let (buf, number_of_locals) = be_u16(buf)?;
                let (buf, locals) =
                    count(VerificationTypeInfo::parse, number_of_locals as usize)(buf)?;
                let (buf, number_of_stack_items) = be_u16(buf)?;
                let (buf, stack) =
                    count(VerificationTypeInfo::parse, number_of_stack_items as usize)(buf)?;
                Ok((
                    buf,
                    StackMapFrame::FullFrame(
                        offset_delta,
                        number_of_locals,
                        locals,
                        number_of_stack_items,
                        stack,
                    ),
                ))
            }
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug)]
pub enum VerificationTypeInfo {
    TopVariableInfo,
    IntegerVariableInfo,
    FloatVariableInfo,
    NullVariableInfo,
    UninitializedThisVariableInfo,
    ObjectVariableInfo(u16),
    UninitializedVariableInfo(u16),
    LongVariableInfo,
    DoubleVariableInfo,
}

impl VerificationTypeInfo {
    fn parse(buf: &[u8]) -> IResult<&[u8], Self> {
        let (buf, tag) = u8(buf)?;
        match tag {
            0 => Ok((buf, VerificationTypeInfo::TopVariableInfo)),
            1 => Ok((buf, VerificationTypeInfo::IntegerVariableInfo)),
            2 => Ok((buf, VerificationTypeInfo::FloatVariableInfo)),
            3 => Ok((buf, VerificationTypeInfo::DoubleVariableInfo)),
            4 => Ok((buf, VerificationTypeInfo::LongVariableInfo)),
            5 => Ok((buf, VerificationTypeInfo::NullVariableInfo)),
            6 => Ok((buf, VerificationTypeInfo::UninitializedThisVariableInfo)),
            7 => {
                let (buf, cpool_index) = be_u16(buf)?;
                Ok((buf, VerificationTypeInfo::ObjectVariableInfo(cpool_index)))
            }
            8 => {
                let (buf, offset) = be_u16(buf)?;
                Ok((buf, VerificationTypeInfo::UninitializedVariableInfo(offset)))
            }
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug)]
pub struct Exceptions {
    pub exception_index_table: Vec<u16>,
}

impl Exceptions {
    fn parse(buf: &[u8]) -> IResult<&[u8], Self> {
        let (buf, number_of_exceptions) = be_u16(buf)?;
        let (buf, exception_index_table) = count(be_u16, number_of_exceptions as usize)(buf)?;
        Ok((
            buf,
            Exceptions {
                exception_index_table,
            },
        ))
    }
}

#[derive(Debug)]
pub struct InnerClasses {
    pub classes: Vec<InnerClass>,
}

impl InnerClasses {
    fn parse(buf: &[u8]) -> IResult<&[u8], Self> {
        let (buf, number_of_classes) = be_u16(buf)?;
        let (buf, classes) = count(InnerClass::parse, number_of_classes as usize)(buf)?;

        Ok((buf, InnerClasses { classes }))
    }
}

#[derive(Debug)]
pub struct InnerClass {
    pub inner_class_info_index: u16,
    pub outer_class_info_index: u16,
    pub inner_name_index: u16,
    pub inner_class_access_flags: u16,
}

impl InnerClass {
    fn parse(buf: &[u8]) -> IResult<&[u8], Self> {
        let (buf, inner_class_info_index) = be_u16(buf)?;
        let (buf, outer_class_info_index) = be_u16(buf)?;
        let (buf, inner_name_index) = be_u16(buf)?;
        let (buf, inner_class_access_flags) = be_u16(buf)?;

        Ok((
            buf,
            InnerClass {
                inner_class_info_index,
                outer_class_info_index,
                inner_name_index,
                inner_class_access_flags,
            },
        ))
    }
}

#[derive(Debug)]
pub struct EnclosingMethod {
    pub class_index: u16,
    pub method_index: u16,
}

impl EnclosingMethod {
    fn parse(buf: &[u8]) -> IResult<&[u8], Self> {
        let (buf, class_index) = be_u16(buf)?;
        let (buf, method_index) = be_u16(buf)?;

        Ok((
            buf,
            EnclosingMethod {
                class_index,
                method_index,
            },
        ))
    }
}

#[derive(Debug)]
pub struct Signature {
    pub signature_index: u16,
}

impl Signature {
    fn parse(buf: &[u8]) -> IResult<&[u8], Self> {
        let (buf, signature_index) = be_u16(buf)?;

        Ok((buf, Signature { signature_index }))
    }
}

#[derive(Debug)]
pub struct SourceFile {
    pub sourcefile_index: u16,
}

impl SourceFile {
    fn parse(buf: &[u8]) -> IResult<&[u8], Self> {
        let (buf, sourcefile_index) = be_u16(buf)?;

        Ok((buf, SourceFile { sourcefile_index }))
    }
}

#[derive(Debug)]
pub struct LineNumber {
    pub start_pc: u16,
    pub line_number: u16,
}

impl LineNumber {
    fn parse(buf: &[u8]) -> IResult<&[u8], Self> {
        let (buf, start_pc) = be_u16(buf)?;
        let (buf, line_number) = be_u16(buf)?;

        Ok((
            buf,
            LineNumber {
                start_pc,
                line_number,
            },
        ))
    }
}

#[derive(Debug)]
pub struct LocalVariable {
    pub start_pc: u16,
    pub length: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub index: u16,
}

impl LocalVariable {
    fn parse(buf: &[u8]) -> IResult<&[u8], Self> {
        let (buf, start_pc) = be_u16(buf)?;
        let (buf, length) = be_u16(buf)?;
        let (buf, name_index) = be_u16(buf)?;
        let (buf, descriptor_index) = be_u16(buf)?;
        let (buf, index) = be_u16(buf)?;

        Ok((
            buf,
            LocalVariable {
                start_pc,
                length,
                name_index,
                descriptor_index,
                index,
            },
        ))
    }
}

#[derive(Debug)]
pub struct LocalVariableType {
    pub start_pc: u16,
    pub length: u16,
    pub name_index: u16,
    pub signature_index: u16,
    pub index: u16,
}

impl LocalVariableType {
    fn parse(buf: &[u8]) -> IResult<&[u8], Self> {
        let (buf, start_pc) = be_u16(buf)?;
        let (buf, length) = be_u16(buf)?;
        let (buf, name_index) = be_u16(buf)?;
        let (buf, signature_index) = be_u16(buf)?;
        let (buf, index) = be_u16(buf)?;

        Ok((
            buf,
            LocalVariableType {
                start_pc,
                length,
                name_index,
                signature_index,
                index,
            },
        ))
    }
}

#[derive(Debug)]
pub struct Annotation {
    pub type_index: u16,
    pub element_value_pairs: Vec<(u16, ElementValue)>,
}

impl Annotation {
    fn parse(buf: &[u8]) -> IResult<&[u8], Self> {
        let (buf, type_index) = be_u16(buf)?;
        let (mut buf, num_element_value_pairs) = be_u16(buf)?;
        let mut element_value_pairs = Vec::with_capacity(num_element_value_pairs as usize);
        for _ in 0..num_element_value_pairs {
            let (temp_buf, element_name_index) = be_u16(buf)?;
            let (temp_buf, element_value) = ElementValue::parse(temp_buf)?;
            buf = temp_buf;
            element_value_pairs.push((element_name_index, element_value));
        }

        Ok((
            buf,
            Annotation {
                type_index,
                element_value_pairs,
            },
        ))
    }
}

#[derive(Debug)]
pub enum ElementValue {
    ConstValue(u16),
    EnumConstValue(u16, u16),
    ClassInfoIndex(u16),
    AnnotationValue(Annotation),
    ArrayValue(Vec<ElementValue>),
}

impl ElementValue {
    fn parse(buf: &[u8]) -> IResult<&[u8], Self> {
        let (buf, tag) = u8(buf)?;
        match tag {
            // B | C | D | F | I | J | S | Z | s
            0x42 | 0x43 | 0x44 | 0x46 | 0x49 | 0x4A | 0x53 | 0x5A | 0x73 => {
                let (buf, const_value_index) = be_u16(buf)?;
                Ok((buf, ElementValue::ConstValue(const_value_index)))
            }
            // e
            0x65 => {
                let (buf, type_name_index) = be_u16(buf)?;
                let (buf, const_name_index) = be_u16(buf)?;
                Ok((
                    buf,
                    ElementValue::EnumConstValue(type_name_index, const_name_index),
                ))
            }
            // c
            0x63 => {
                let (buf, class_info_index) = be_u16(buf)?;
                Ok((buf, ElementValue::ClassInfoIndex(class_info_index)))
            }
            // @
            0x40 => {
                let (buf, annotation) = Annotation::parse(buf)?;
                Ok((buf, ElementValue::AnnotationValue(annotation)))
            }
            // {
            0x5b => {
                let (buf, num_values) = be_u16(buf)?;
                let (buf, array) = count(ElementValue::parse, num_values as usize)(buf)?;
                Ok((buf, ElementValue::ArrayValue(array)))
            }
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug)]
pub struct ParameterAnnotation {
    pub annotations: Vec<Annotation>,
}

impl ParameterAnnotation {
    fn parse(buf: &[u8]) -> IResult<&[u8], Self> {
        let (buf, num_annotations) = be_u16(buf)?;
        let (buf, annotations) = count(Annotation::parse, num_annotations as usize)(buf)?;

        Ok((buf, ParameterAnnotation { annotations }))
    }
}

#[derive(Debug)]
pub struct TypeAnnotation {
    target_type: u8,
    target_info: TargetInfo,
    target_path: TypePath,
    type_index: u16,
    element_value_pairs: Vec<(u16, ElementValue)>,
}

impl TypeAnnotation {
    fn parse(buf: &[u8]) -> IResult<&[u8], Self> {
        let (buf, target_type) = u8(buf)?;
        let (buf, target_info) = TargetInfo::parse(buf, target_type)?;
        let (buf, target_path) = TypePath::parse(buf)?;
        let (buf, type_index) = be_u16(buf)?;
        let (mut buf, num_element_value_pairs) = be_u16(buf)?;
        let mut element_value_pairs = Vec::with_capacity(num_element_value_pairs as usize);
        for _ in 0..num_element_value_pairs {
            let (temp_buf, element_name_index) = be_u16(buf)?;
            let (temp_buf, element_value) = ElementValue::parse(temp_buf)?;
            buf = temp_buf;
            element_value_pairs.push((element_name_index, element_value));
        }

        Ok((
            buf,
            TypeAnnotation {
                target_type,
                target_info,
                target_path,
                type_index,
                element_value_pairs,
            },
        ))
    }
}

#[derive(Debug)]
pub enum TargetInfo {
    TypeParameter(u8),
    SuperType(u16),
    TypeParameterBound(u8, u8),
    Empty,
    FormalParameter(u8),
    Throws(u16),
    LocalVar(Vec<LocalVar>),
    Catch(u16),
    Offset(u16),
    TypeArgument(u16, u8),
}

#[derive(Debug)]
pub struct LocalVar {
    start_pc: u16,
    length: u16,
    index: u16,
}

impl LocalVar {
    fn parse(buf: &[u8]) -> IResult<&[u8], Self> {
        let (buf, start_pc) = be_u16(buf)?;
        let (buf, length) = be_u16(buf)?;
        let (buf, index) = be_u16(buf)?;

        Ok((
            buf,
            LocalVar {
                start_pc,
                length,
                index,
            },
        ))
    }
}

impl TargetInfo {
    fn parse(buf: &[u8], target_type: u8) -> IResult<&[u8], Self> {
        match target_type {
            0 | 1 => {
                let (buf, type_parameter_index) = u8(buf)?;
                Ok((buf, TargetInfo::TypeParameter(type_parameter_index)))
            }
            0x10 => {
                let (buf, supertype_index) = be_u16(buf)?;
                Ok((buf, TargetInfo::SuperType(supertype_index)))
            }
            0x11 | 0x12 => {
                let (buf, type_parameter_index) = u8(buf)?;
                let (buf, bound_index) = u8(buf)?;
                Ok((
                    buf,
                    TargetInfo::TypeParameterBound(type_parameter_index, bound_index),
                ))
            }
            0x13 | 0x14 | 0x15 => Ok((buf, TargetInfo::Empty)),
            0x16 => {
                let (buf, formal_parameter_index) = u8(buf)?;
                Ok((buf, TargetInfo::FormalParameter(formal_parameter_index)))
            }
            0x17 => {
                let (buf, throws_type_index) = be_u16(buf)?;
                Ok((buf, TargetInfo::Throws(throws_type_index)))
            }
            0x40 | 0x41 => {
                let (buf, table_length) = be_u16(buf)?;
                let (buf, table) = count(LocalVar::parse, table_length as usize)(buf)?;
                Ok((buf, TargetInfo::LocalVar(table)))
            }
            0x42 => {
                let (buf, exception_table_index) = be_u16(buf)?;
                Ok((buf, TargetInfo::Catch(exception_table_index)))
            }
            0x43 | 0x44 | 0x45 | 0x46 => {
                let (buf, offset) = be_u16(buf)?;
                Ok((buf, TargetInfo::Offset(offset)))
            }
            0x47 | 0x48 | 0x49 | 0x4A | 0x4B => {
                let (buf, offset) = be_u16(buf)?;
                let (buf, type_argument_index) = u8(buf)?;
                Ok((buf, TargetInfo::TypeArgument(offset, type_argument_index)))
            }
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug)]
pub struct TypePath {
    pub path: Vec<Path>,
}

impl TypePath {
    fn parse(buf: &[u8]) -> IResult<&[u8], Self> {
        let (buf, path_length) = u8(buf)?;
        let (buf, path) = count(Path::parse, path_length as usize)(buf)?;

        Ok((buf, TypePath { path }))
    }
}

#[derive(Debug)]
pub struct Path {
    pub type_path_kind: u8,
    pub type_argument_index: u8,
}

impl Path {
    fn parse(buf: &[u8]) -> IResult<&[u8], Self> {
        let (buf, type_path_kind) = u8(buf)?;
        let (buf, type_argument_index) = u8(buf)?;

        Ok((
            buf,
            Path {
                type_path_kind,
                type_argument_index,
            },
        ))
    }
}

#[derive(Debug)]
pub struct BootstrapMethod {
    pub bootstrap_method_ref: u16,
    pub bootstrap_arguments: Vec<u16>,
}

impl BootstrapMethod {
    fn parse(buf: &[u8]) -> IResult<&[u8], Self> {
        let (buf, bootstrap_method_ref) = be_u16(buf)?;
        let (buf, num_bootstrap_arguments) = be_u16(buf)?;
        let (buf, bootstrap_arguments) = count(be_u16, num_bootstrap_arguments as usize)(buf)?;

        Ok((
            buf,
            BootstrapMethod {
                bootstrap_method_ref,
                bootstrap_arguments,
            },
        ))
    }
}

#[derive(Debug)]
pub struct Parameter {
    pub name_index: u16,
    pub access_flags: u16,
}

impl Parameter {
    fn parse(buf: &[u8]) -> IResult<&[u8], Self> {
        let (buf, name_index) = be_u16(buf)?;
        let (buf, access_flags) = be_u16(buf)?;

        Ok((
            buf,
            Parameter {
                name_index,
                access_flags,
            },
        ))
    }
}

#[derive(Debug)]
pub struct Module {
    pub module_name_index: u16,
    pub module_flags: u16,
    pub module_version_index: u16,
    pub requires: Vec<Requires>,
    pub exports: Vec<Exports>,
    pub opens: Vec<Opens>,
    pub uses: Vec<u16>,
    pub provides: Vec<Provides>,
}

impl Module {
    fn parse(buf: &[u8]) -> IResult<&[u8], Self> {
        let (buf, module_name_index) = be_u16(buf)?;
        let (buf, module_flags) = be_u16(buf)?;
        let (buf, module_version_index) = be_u16(buf)?;
        let (buf, requires_count) = be_u16(buf)?;
        let (buf, requires) = count(Requires::parse, requires_count as usize)(buf)?;
        let (buf, exports_count) = be_u16(buf)?;
        let (buf, exports) = count(Exports::parse, exports_count as usize)(buf)?;
        let (buf, opens_count) = be_u16(buf)?;
        let (buf, opens) = count(Opens::parse, opens_count as usize)(buf)?;
        let (buf, uses_count) = be_u16(buf)?;
        let (buf, uses) = count(be_u16, uses_count as usize)(buf)?;
        let (buf, provides_count) = be_u16(buf)?;
        let (buf, provides) = count(Provides::parse, provides_count as usize)(buf)?;

        Ok((
            buf,
            Module {
                module_name_index,
                module_flags,
                module_version_index,
                requires,
                exports,
                opens,
                uses,
                provides,
            },
        ))
    }
}

#[derive(Debug)]
pub struct Requires {
    pub requires_index: u16,
    pub requires_flags: u16,
    pub requires_version_index: u16,
}

impl Requires {
    fn parse(buf: &[u8]) -> IResult<&[u8], Self> {
        let (buf, requires_index) = be_u16(buf)?;
        let (buf, requires_flags) = be_u16(buf)?;
        let (buf, requires_version_index) = be_u16(buf)?;

        Ok((
            buf,
            Requires {
                requires_index,
                requires_flags,
                requires_version_index,
            },
        ))
    }
}

#[derive(Debug)]
pub struct Exports {
    pub exports_index: u16,
    pub exports_flags: u16,
    pub exports_to_index: Vec<u16>,
}
impl Exports {
    fn parse(buf: &[u8]) -> IResult<&[u8], Self> {
        let (buf, exports_index) = be_u16(buf)?;
        let (buf, exports_flags) = be_u16(buf)?;
        let (buf, exports_to_count) = be_u16(buf)?;
        let (buf, exports_to_index) = count(be_u16, exports_to_count as usize)(buf)?;

        Ok((
            buf,
            Exports {
                exports_index,
                exports_flags,
                exports_to_index,
            },
        ))
    }
}

#[derive(Debug)]
pub struct Opens {
    pub opens_index: u16,
    pub opens_flags: u16,
    pub opens_to_index: Vec<u16>,
}
impl Opens {
    fn parse(buf: &[u8]) -> IResult<&[u8], Self> {
        let (buf, opens_index) = be_u16(buf)?;
        let (buf, opens_flags) = be_u16(buf)?;
        let (buf, opens_to_count) = be_u16(buf)?;
        let (buf, opens_to_index) = count(be_u16, opens_to_count as usize)(buf)?;

        Ok((
            buf,
            Opens {
                opens_index,
                opens_flags,
                opens_to_index,
            },
        ))
    }
}

#[derive(Debug)]
pub struct Provides {
    pub provides_index: u16,
    pub provides_with_index: Vec<u16>,
}

impl Provides {
    fn parse(buf: &[u8]) -> IResult<&[u8], Self> {
        let (buf, provides_index) = be_u16(buf)?;
        let (buf, provides_to_count) = be_u16(buf)?;
        let (buf, provides_with_index) = count(be_u16, provides_to_count as usize)(buf)?;

        Ok((
            buf,
            Provides {
                provides_index,
                provides_with_index,
            },
        ))
    }
}

#[derive(Debug)]
pub struct RecordComponentInfo {
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes: Vec<Attribute>,
}

impl RecordComponentInfo {
    fn parse<'a>(buf: &'a [u8], constant_pool: &[ConstantPool]) -> IResult<&'a [u8], Self> {
        let (buf, name_index) = be_u16(buf)?;
        let (buf, descriptor_index) = be_u16(buf)?;
        let (buf, attributes_count) = be_u16(buf)?;
        let (buf, attributes) = count(AttributeInfo::parse, attributes_count as usize)(buf)?;
        let attributes = Attribute::from_attribute_info(attributes, &constant_pool);

        Ok((
            buf,
            RecordComponentInfo {
                name_index,
                descriptor_index,
                attributes,
            },
        ))
    }
}
