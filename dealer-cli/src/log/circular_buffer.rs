use std::{collections::VecDeque, sync::RwLock};

#[derive(Debug)]
pub struct CircularBuf<T> {
    buf: RwLock<VecDeque<T>>,
    capacity: usize,
}

impl<T> CircularBuf<T> {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "Capacity must be greater than 0");
        CircularBuf {
            buf: RwLock::new(VecDeque::with_capacity(capacity)),
            capacity,
        }
    }

    pub fn push(&self, value: T) {
        let mut buf = self.buf.write().unwrap();
        if buf.len() == self.capacity {
            buf.pop_front();
        }
        buf.push_back(value);
    }

    // pub fn get_all(&self) -> Vec<T>
    // where
    //     T: Clone,
    // {
    //     let buf = self.buf.read().unwrap();
    //     buf.clone().into()
    // }

    pub fn get(&self, size: usize) -> Vec<T>
    where
        T: Clone,
    {
        let buf = self.buf.read().unwrap();
        let start = if buf.len() < size {
            0
        } else {
            buf.len() - size
        };

        buf.iter().skip(start).cloned().collect()
    }
}