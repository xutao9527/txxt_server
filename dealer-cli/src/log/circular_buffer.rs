use std::collections::VecDeque;

#[derive(Debug)]
struct CircularBuf<T> {
    buf: VecDeque<T>,
    capacity: usize,
}

impl<T> CircularBuf<T> {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "Capacity must be greater than 0");
        CircularBuf {
            buf: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    pub fn push(&mut self, value: T) {
        if self.buf.len() == self.capacity {
            // 如果队列已满，移除最旧的元素
            self.buf.pop_front();
        }
        // 将新元素推入队列
        self.buf.push_back(value);
    }

    pub fn get_all(&self) -> Vec<&T> {
        self.buf.iter().collect()
    }

    pub fn get(&self, size: usize) -> Vec<&T> {
        let start = if self.buf.len() < size {
            0
        } else {
            self.buf.len() - size
        };

        self.buf.iter().skip(start).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::log::circular_buffer::CircularBuf;
    #[test]
    fn test_circular_buffer() {
        let mut buffer = CircularBuf::<i32>::new(5);
        buffer.push(1);
        buffer.push(2);
        buffer.push(3);
        println!("{:?}", buffer.get_all()); // 输出: [1, 2, 3]

        buffer.push(4);
        println!("{:?}", buffer.get_all()); // 输出: [2, 3, 4]
        
        buffer.push(5);
        println!("{:?}", buffer.get_all()); // 输出: [3, 4, 5]
        println!("{:?}", buffer.get(4)); // 输出: [3, 4, 5]
    }
}
