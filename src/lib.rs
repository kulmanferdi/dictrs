use std::collections::VecDeque;
use std::fmt::Debug;

#[derive(Debug)]
pub struct InvalidKeyException;

#[derive(Clone)]
pub struct Dictionary<K, V>{
    items: VecDeque<Item<K, V>>,
}

#[derive(Clone)]
pub struct Item<K, V>{
    key: K,
    value: V,
}

impl<K, V> Dictionary<K, V>
where
    K: PartialEq + Debug,
{
    pub fn new() -> Dictionary<K, V> {
        Dictionary {
            items: VecDeque::new(),
        }
    }

    pub fn empty(&mut self) {
        self.items.clear()
    }
    
    pub fn count(&self) -> usize {
        self.items.len()
    }
    
    pub fn contains(&self, key: &K) -> bool {
        let mut i: usize = 0;
        while i < self.items.len() && self.items[i].key != *key {
            i += 1;
        }
        i < self.items.len()
    }
    
    pub fn insert(&mut self, key: K, value: V) {
        let mut i: usize = 0;
        while i < self.items.len() && self.items[i].key != key {
            i += 1;
        }
        if i < self.items.len() {
            self.items[i].value = value;
        }
        else {
            self.items.push_back(Item { key, value});
        } 
    }
    
    pub fn delete(&mut self, key: &K) -> Result<(), InvalidKeyException> {
        let mut i: usize = 0;
        while i < self.items.len() && self.items[i].key != *key {
            i += 1;
        }
        if i < self.items.len() {
            self.items.remove(i);
            Ok(())
        }
        else  {
            Err(InvalidKeyException)
        }
    }
}