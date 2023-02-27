use std::collections::BTreeMap;

use std::fs::File;
use std::io::{Read, Write};


pub struct Storage {
    data: BTreeMap<String, String>,
}

impl Storage {
    pub fn new() -> Storage {
        Storage {
            data: BTreeMap::new()
        }
    }
    /// set an value,return an old value.
    pub fn set(&mut self, key: &str, value: &str) -> Option<String> {
        let old_val = self.data.get(key).cloned();
        self.data.insert(key.parse().unwrap(), value.parse().unwrap());
        old_val
    }
    pub fn get(&self, key: &str) -> &str {
        match self.data.get(key) {
            None => { "" }
            Some(val) => { val }
        }
    }
    pub fn save(&self) -> bool {
        //storage each k,v in to json
        let data_str = json::stringify(self.data.clone());
        //write data_str into data.json
        let mut file = File::create("data.json").unwrap();
        let save_result = file.write_all(data_str.as_bytes());
        match save_result {
            Ok(_) => {
                true
            }
            Err(_) => {
                false
            }
        }
    }
    pub fn load(&mut self)
    {
        let mut file = File::open("data.json").unwrap();
        let mut content_str = String::new();
        file.read_to_string(&mut content_str).unwrap();

        let obj = json::parse(content_str.as_str()).unwrap();
        let ent = obj.entries();
        for (k, v) in ent {
            self.data.insert(k.to_string(), v.as_str().unwrap().to_string()).unwrap();
        }
    }
    pub fn remove(&self) {}
}