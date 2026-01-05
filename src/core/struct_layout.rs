use std::collections::HashMap;
use chumsky::container::Container;

pub struct StructLayout {
    fields: HashMap<String, Field>,
}

pub struct Field {
    pub(crate) start_offs: usize,
    pub(crate) size: usize,
}

impl StructLayout {
    pub fn size(&self) -> usize {
        self.fields
            .values()
            .map(|f| f.start_offs + f.size)
            .max()
            .unwrap_or(0)
    }

    pub fn store_indirect(&self) -> bool {
        self.size() > 16
    }

    pub fn push(&mut self, name: String, offs: usize, size: usize) {
        self.fields.push((name, Field {start_offs: offs, size}));
    }
}