#[derive(Default)]
pub struct IdPool {
    free_ids: Vec<u32>,
    next_id: u32
}

impl IdPool {
    pub fn new() -> IdPool {
        IdPool {
            free_ids: Vec::new(),
            next_id: 0
        }
    }

    pub fn get(&mut self) -> u32 {
        if !self.free_ids.is_empty() {
            self.free_ids.pop().unwrap()
        } else {
            self.next_id += 1;
            self.next_id
        }
    }

    pub fn free(&mut self, id: u32) {
        self.free_ids.push(id);
    }
}