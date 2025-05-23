use crate::types::IdType;

#[derive(Clone)]
pub struct Dictionary {
    strings: Vec<String>,
    id_map: std::collections::HashMap<String, IdType>,
    next_id: IdType,
}

impl Dictionary {
    pub fn new() -> Self {
        Self {
            strings: Vec::new(),
            id_map: std::collections::HashMap::new(),
            next_id: 0,
        }
    }

    pub fn insert(&mut self, string: String) -> IdType {
        if let Some(&id) = self.id_map.get(&string) {
            return id;
        }

        let id = self.next_id;
        self.next_id += 1;
        self.strings.push(string.clone());
        self.id_map.insert(string, id);
        id
    }

    pub fn get(&self, id: IdType) -> Option<&str> {
        self.strings.get(id as usize).map(|s| s.as_str())
    }

    pub fn get_id(&self, string: &str) -> Option<IdType> {
        self.id_map.get(string).copied()
    }

    pub fn len(&self) -> usize {
        self.strings.len()
    }

    pub fn is_empty(&self) -> bool {
        self.strings.is_empty()
    }
} 