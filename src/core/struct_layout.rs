use std::collections::HashMap;

pub struct StructLayout {
    fields: HashMap<String, Field>,
}

pub struct Field {
    pub(crate) start_offs: usize,
    pub(crate) size: usize,
}

impl Field {
    pub(crate) fn endpoint(&self) -> usize {
        self.start_offs + self.size
    }
}

impl StructLayout {
    pub fn size(&self) -> usize {
        self.fields
            .values()
            .map(Field::endpoint)
            .max()
            .unwrap_or(0)
    }

    pub fn store_indirect(&self) -> bool {
        self.size() > 16
    }

    pub fn push(&mut self, name: String, offs: usize, size: usize) {
        self.fields.insert(name, Field {start_offs: offs, size});
    }

    pub fn get(&self, name: &str) -> Option<&Field> {
        self.fields.get(name)
    }
}