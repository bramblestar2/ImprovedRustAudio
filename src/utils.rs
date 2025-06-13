use std::{cmp::Reverse, collections::BinaryHeap, iter::Rev};

#[derive(Default)]
pub struct IdPool {
    free_ids: BinaryHeap<Reverse<u32>>,
    next_id: u32
}

impl IdPool {
    pub fn new() -> IdPool {
        IdPool {
            free_ids: BinaryHeap::new(),
            next_id: 1
        }
    }

    pub fn get(&mut self) -> u32 {
        if let Some(Reverse(id)) = self.free_ids.pop() {
            id
        } else {
            let id = self.next_id;
            self.next_id = self.next_id
                .checked_add(1)
                .expect("IdPool exhausted u32 range");
            id
        }
    }

    pub fn reserve(&mut self, id: u32) {
        if id >= self.next_id {
            self.next_id = id
                .checked_add(1)
                .expect("IdPool exhausted u32 range");
        } else {
            let mut buf = BinaryHeap::new();
            while let Some(Reverse(x)) = self.free_ids.pop() {
                if x != id {
                    buf.push(Reverse(x));
                }
            }

            self.free_ids = buf;
        }
    }

    pub fn free(&mut self, id: u32) {
        if id < self.next_id && !self.free_ids.iter().any(|&Reverse(x)| x == id) {
            self.free_ids.push(Reverse(id));
        }
    }
}