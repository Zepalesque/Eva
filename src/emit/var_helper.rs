use std::collections::HashMap;
use crate::core::RegIdRepr;

pub struct VarHelper {
    map: HashMap<String, Vec<RegIdRepr>>,
    curr: RegIdRepr,
}

impl VarHelper {
    pub fn new() -> Self {
        Self {
            map: HashMap::<String, Vec<RegIdRepr>>::new(),
            curr: 0,
        }
    }

    pub fn alias(&mut self, original: &str, alias: String) -> Option<RegIdRepr> {
        let id = self.get(original)?;

        self.map.entry(alias).or_default().push(id);
        
        Some(id)
    }

    pub fn get(&self, name: &str) -> Option<RegIdRepr> {
        self.map.get(name)?.last().copied()
    }

    pub fn add_or_shadow(&mut self, name: &str) -> RegIdRepr {
        let id = self.next();
        self.map.entry(name.to_string()).or_default().push(id);
        id
    }

    pub fn add_or_shadow_128(&mut self, name: &str) -> RegIdRepr {
        let id = self.next_n(2);
        self.map.entry(name.to_string()).or_default().push(id);
        id
    }



    pub fn create_temp(&mut self) -> RegIdRepr {
        let id = self.next();
        let name: String = format!("__var{}", id);
        self.map.entry(name).or_default().push(id);

        id
    }

    fn next(&mut self) -> RegIdRepr {
        let res = self.curr;
        self.curr = self.curr.checked_add(1)
            .expect("Stack Overflow-ish: Too many register ids!");
        res
    }

    fn next_n(&mut self, reg_count: RegIdRepr) -> RegIdRepr {
        let res = self.curr;
        self.curr = self.curr.checked_add(reg_count)
            .expect("Stack Overflow-ish: Too many register ids!");
        res
    }
}