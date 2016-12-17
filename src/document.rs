use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::fmt::Error as FmtError;
use std::rc::Rc;
use std::ops::Deref;
use errors::*;
use std::result::Result as StdResult;

pub type Namespaces = BTreeMap<Rc<String>, Rc<String>>;
#[derive(Default, Debug)]
pub struct Document {
    pub header: Header,
    pub header_string_table: HeaderStringTable,
    pub header_resource_table: HeaderResourceTable,
    pub header_namespace: HeaderNamespace,

    pub string_table: StringTable,
    pub resource_table: ResourceTable,
    pub resources: Namespaces,
    pub root_element: Element,
}

pub struct Package {
    pub name: String,
    pub type_string_table: Option<StringTable>,
    pub key_string_table: Option<StringTable>,
}

impl Package {
    pub fn new(name: String) -> Self {
        Package {
            name: name,
            type_string_table: None,
            key_string_table: None,
        }
    }
}

#[derive(Default, Debug)]
pub struct Header {
    pub size: u32,
}

#[derive(Default, Debug)]
pub struct HeaderStringTable {
    pub chunk: u32,
    pub string_amount: u32,
    pub style_amount: u32,
    pub flags: u32,
    pub string_offset: u32,
    pub style_offset: u32,
}

#[derive(Default, Debug)]
pub struct StringTable {
    pub strings: Vec<Rc<String>>,
    pub styles: Vec<Rc<String>>,
}

#[derive(Default, Debug)]
pub struct HeaderResourceTable {
    pub chunk: u32,
}

#[derive(Default, Debug)]
pub struct ResourceTable {
    pub resources: Vec<u32>,
}

#[derive(Default, Debug)]
pub struct HeaderNamespace {
    pub chunk: u32,
}

#[derive(Default, Debug)]
pub struct Element {
    tag: Rc<String>,
    attrs: Vec<Attribute>,
    children: Vec<Element>,
    level: u32,
}

impl Element {
    pub fn new(tag: Rc<String>, attrs: Vec<Attribute>) -> Self {
        Element {
            tag: tag,
            attrs: attrs,
            children: Vec::new(),
            level: 0,
        }
    }

    pub fn append(&mut self, element: Element) {
        self.children.push(element)
    }

    pub fn set_level(&mut self, level: u32) {
        self.level = level;
    }

    pub fn get_attributes(&self) -> &Vec<Attribute> {
        &self.attrs
    }

    pub fn get_tag(&self) -> Rc<String> {
        self.tag.clone()
    }

    pub fn get_children(&self) -> &Vec<Element> {
        &self.children
    }
}

impl Display for Element {
    fn fmt(&self, formatter: &mut Formatter) -> StdResult<(), FmtError> {
        let tabs = "\t".to_string().repeat(self.level as usize);
        write!(formatter, "{}Element: {}\n", tabs, self.tag)?;

        for c in &self.children {
            write!(formatter, "{}", c)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub enum Value {
    String(Rc<String>),
    Dimension(String),
    Fraction(String),
    Float(f64),
    Integer(u64),
    Flags(u64),
    Boolean(bool),
    Color(String),
    Color2(String),
    ReferenceId(String),
    AttributeReferenceId(String),
    Unknown,
}

const TOKEN_VOID: u32 = 0xFFFFFFFF;

const TOKEN_TYPE_REFERENCE_ID: u32 = 0x01000008;
const TOKEN_TYPE_ATTRIBUTE_REFERENCE_ID: u32 = 0x02000008;
const TOKEN_TYPE_STRING: u32 = 0x03000008;
const TOKEN_TYPE_DIMENSION: u32 = 0x05000008;
const TOKEN_TYPE_FRACTION: u32 = 0x06000008;
const TOKEN_TYPE_INTEGER: u32 = 0x10000008;
const TOKEN_TYPE_FLOAT: u32 = 0x04000008;
const TOKEN_TYPE_FLAGS: u32 = 0x11000008;
const TOKEN_TYPE_BOOLEAN: u32 = 0x12000008;
const TOKEN_TYPE_COLOR: u32 = 0x1C000008;
const TOKEN_TYPE_COLOR2: u32 = 0x1D000008;

impl Value {
    pub fn to_string(&self) -> String {
        match self {
            &Value::String(ref s) => s.deref().clone(),
            &Value::Dimension(ref s) => s.clone(),
            &Value::Fraction(ref s) => s.clone(),
            &Value::Float(f) => f.to_string(),
            &Value::Integer(i) => i.to_string(),
            &Value::Flags(i) => i.to_string(),
            &Value::Boolean(b) => b.to_string(),
            &Value::Color(ref s) => s.clone(),
            &Value::Color2(ref s) => s.clone(),
            &Value::ReferenceId(ref s) => s.clone(),
            &Value::AttributeReferenceId(ref s) => s.clone(),
            _ => "Unknown".to_string(),
        }
    }

    pub fn new(value_type: u32, data: u32, str_table: &StringTable) -> Result<Self> {
        let value = match value_type {
            TOKEN_TYPE_REFERENCE_ID => Value::ReferenceId(format!("@id/0x{:#8}", data)),
            TOKEN_TYPE_ATTRIBUTE_REFERENCE_ID => {
                Value::AttributeReferenceId(format!("?id/0x{:#8}", data))
            }
            TOKEN_TYPE_STRING => {
                Value::String(str_table
                    .strings
                    .get(data as usize)
                    .unwrap()
                    .clone()
                )
            }
            TOKEN_TYPE_DIMENSION => {
                let units: [&str; 6] = ["px", "dp", "sp", "pt", "in", "mm"];
                let mut size = (data >> 8).to_string();
                let unit_idx = data & 0xFF;

                match units.get(unit_idx as usize) {
                    Some(unit) => size.push_str(unit),
                    None => {
                        return Err(format!("Expected a valid unit index. Got: {}",
                                unit_idx).into())
                    }
                }

                Value::Dimension(size)
            }
            TOKEN_TYPE_FRACTION => {
                let value = (data as f64) / (0x7FFFFFFF as f64);
                let formatted_fraction = format!("{:.*}", 2, value);

                Value::Fraction(formatted_fraction)
            }
            TOKEN_TYPE_INTEGER => Value::Integer(data as u64),
            TOKEN_TYPE_FLAGS => Value::Flags(data as u64),
            TOKEN_TYPE_FLOAT => Value::Float(data as f64),
            TOKEN_TYPE_BOOLEAN => {
                if data > 0 {
                    Value::Boolean(true)
                } else {
                    Value::Boolean(false)
                }
            }
            TOKEN_TYPE_COLOR => {
                let formatted_color = format!("#{:#8}", data);
                Value::Color(formatted_color)
            }
            TOKEN_TYPE_COLOR2 => {
                let formatted_color = format!("#{:#8}", data);
                Value::Color2(formatted_color)
            }
            _ => Value::Unknown,

        };

        Ok(value)
    }
}

#[derive(Debug)]
pub struct Attribute {
    name: Rc<String>,
    namespace: Option<Rc<String>>,
    prefix: Option<Rc<String>>,
    value: Value,
}

impl Attribute {
    pub fn new(name: Rc<String>,
               value: Value,
               namespace: Option<Rc<String>>,
               prefix: Option<Rc<String>>)
               -> Self {
        Attribute {
            name: name,
            namespace: namespace,
            prefix: prefix,
            value: value,
        }
    }

    pub fn get_name(&self) -> Rc<String> {
        self.name.clone()
    }

    pub fn get_value(&self) -> String {
        self.value.to_string()
    }

    pub fn get_prefix(&self) -> Option<Rc<String>> {
        self.prefix.clone()
    }
}

pub struct ElementContainer {
    stack: Vec<Element>,
    root: Option<Element>,
}

impl ElementContainer {
    pub fn new() -> Self {
        ElementContainer {
            stack: Vec::new(),
            root: None,
        }
    }

    pub fn start_element(&mut self, mut element: Element) {
        element.set_level(self.stack.len() as u32);
        self.stack.push(element);
    }

    pub fn end_element(&mut self) {
        let element = self.stack.pop().unwrap();

        if self.stack.len() == 0 {
            self.root = Some(element);
        } else {
            // Append child to current element
            let last_element = self.stack.len();
            self.stack[last_element - 1].append(element);
        }
    }

    pub fn get_root(self) -> Option<Element> {
        self.root
    }
}
