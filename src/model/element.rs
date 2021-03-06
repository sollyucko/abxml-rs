use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
    iter,
    rc::Rc,
};

use log::error;

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct Tag {
    name: Rc<String>,
    prefixes: Vec<Rc<String>>,
}

impl Tag {
    pub fn new(name: Rc<String>, prefixes: Vec<Rc<String>>) -> Self {
        Self { name, prefixes }
    }

    pub fn get_name(&self) -> Rc<String> {
        self.name.clone()
    }

    pub fn get_prefixes(&self) -> &Vec<Rc<String>> {
        &self.prefixes
    }
}

#[derive(Default, Debug)]
pub struct Element {
    tag: Tag,
    attrs: HashMap<String, String>,
    children: Vec<Element>,
    level: u32,
}

impl Element {
    pub fn new(tag: Tag, attrs: HashMap<String, String>) -> Self {
        Self {
            tag,
            attrs,
            children: Vec::new(),
            level: 0,
        }
    }

    pub fn append(&mut self, element: Self) {
        self.children.push(element)
    }

    pub fn set_level(&mut self, level: u32) {
        self.level = level;
    }

    pub fn get_attributes(&self) -> &HashMap<String, String> {
        &self.attrs
    }

    pub fn get_tag(&self) -> &Tag {
        &self.tag
    }

    pub fn get_children(&self) -> &Vec<Self> {
        &self.children
    }
}

impl Display for Element {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error> {
        let tabs = iter::repeat("\t")
            .take(self.level as usize)
            .collect::<String>();
        writeln!(formatter, "{}Element: {}", tabs, self.tag.get_name())?;

        for c in &self.children {
            write!(formatter, "{}", c)?;
        }

        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct ElementContainer {
    stack: Vec<Element>,
    root: Option<Element>,
}

impl ElementContainer {
    pub fn start_element(&mut self, mut element: Element) {
        element.set_level(self.stack.len() as u32);
        self.stack.push(element);
    }

    pub fn end_element(&mut self) {
        self.stack
            .pop()
            .and_then(|element| {
                if self.stack.is_empty() {
                    self.root = Some(element);
                } else {
                    // Append child to current element
                    let last_element = self.stack.len();
                    self.stack[last_element - 1].append(element);
                }

                Some(())
            })
            .unwrap_or_else(|| {
                error!("Received an end element event with an empty stack");
            });
    }

    pub fn get_root(&self) -> &Option<Element> {
        &self.root
    }
}
