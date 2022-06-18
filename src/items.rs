use chrono::Utc;
use chrono::DateTime;

// Struct for individual todo entries
#[derive(Clone)]
pub struct Entry {
    content: String,
    date: DateTime<Utc>,
}

impl Entry {
    fn new(content: String) -> Self {
        Self {
            content,
            date: Utc::now(),
        }
    }
    pub fn get_content(&self) -> String {
        self.content.clone()
    }
}

// Struct for the list of todo items
pub struct Items {
    entries: Vec<Entry>,
}

impl Items {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
    pub fn add_entry(&mut self, content: String) {
       self.entries.push(Entry::new(content));
    }
    pub fn get_entries(&self) -> Vec<Entry> {
        self.entries.clone()
    }
    pub fn del_entry(&mut self, index: usize) -> Result<(), ()> {
       if self.entries.len() >= index {
           self.entries.remove(index);
           Ok(())
       } else {
           Err(())
       }
    }
}
