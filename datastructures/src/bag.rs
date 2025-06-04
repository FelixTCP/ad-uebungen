struct bag<T> {
    items: Vec<T>,
    capacity: usize,
}

impl<T> bag<T> {
    fn empty(capacity: usize) -> Self {
        bag {
            items: Vec::with_capacity(capacity),
            capacity,
        }
    }

    fn count(&self, x: T) -> usize {
        self.items.iter().filter(|&&item| item == x).count()
    }

    fn add(&mut self, x: T) {
        if self.items.len() < self.capacity {
            self.items.push(x);
        } else {
            self.capacity *= 2;
            items.reserve(self.capacity);
            self.items.push(x);
        }
    }

    fn member(&self, x: T) -> bool {
        self.items.contains(&x)
    }

    fn remove(&mut self, x: T) {
        self.items.remove(pos);
    }

    fn size(&self) -> usize {
        self.items.len()
    }

    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}
