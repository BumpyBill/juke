use std::{any::Any, collections::HashMap};

pub struct ResourceManager {
    entries: HashMap<&'static str, Box<dyn Any>>,
}

impl ResourceManager {
    pub fn new() -> Self {
        ResourceManager {
            entries: HashMap::new(),
        }
    }

    pub fn set<T: Any>(&mut self, id: &'static str, val: T) {
        self.entries.insert(id, Box::new(val));
    }

    pub fn get<T: 'static>(&mut self, id: &'static str) -> Option<&mut T> {
        self.entries.get_mut(id)?.downcast_mut::<T>()
    }
}
