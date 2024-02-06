pub struct RingBuffer<T> {
    // TODO: fill this in.
    buffer: Vec<T>,
    read_index: usize,
    write_index: usize,

}

impl<T: Copy + Default> RingBuffer<T> {
    pub fn new(length: usize) -> Self {
        // Create a new RingBuffer with `length` slots and "default" values.
        // Hint: look into `vec!` and the `Default` trait.
        let buffer = Vec::with_capacity(length);
        let read_index = 0;
        let write_index = 0;
        RingBuffer { buffer, read_index, write_index }
    }

    pub fn reset(&mut self) {
        // Clear internal buffer and reset indices.
        self.buffer.clear();
        self.read_index = 0;
        self.write_index = 0;
    }

    // `put` and `peek` write/read without advancing the indices.
    pub fn put(&mut self, value: T) {
        if self.write_index < self.buffer.capacity() {
            self.buffer.push(value);
            self.write_index += 1;
        } else {
            println!("Buffer is full, cannot put more elements.");
        }
    }

    pub fn peek(&self) -> Option<&T> {
        if self.read_index < self.write_index {
            Some(&self.buffer[self.read_index])
        } else {
            None
        }
    }

    pub fn get(&self, offset: usize) -> Option<&T> {
        let index = self.read_index + offset;
        if index < self.write_index {
            Some(&self.buffer[index])
        } else {
            None
        }
    }

    // `push` and `pop` write/read and advance the indices.
    pub fn push(&mut self, value: T) {
        if self.write_index < self.buffer.capacity() {
            self.buffer.push(value);
            self.write_index += 1;
        } else {
            println!("Buffer is full, cannot push more elements.");
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.read_index < self.write_index {
            let value = std::mem::replace(&mut self.buffer[self.read_index], Default::default());
            self.read_index += 1;
            Some(value)
        }else {
            None
        }
    }

    pub fn get_read_index(&self) -> usize {
        self.read_index
    }

    pub fn set_read_index(&mut self, index: usize) {
        self.read_index = index;
    }

    pub fn get_write_index(&self) -> usize {
        self.write_index
    }

    pub fn set_write_index(&mut self, index: usize) {
        self.write_index = index;
    }

    pub fn len(&self) -> usize {
        // Return number of values currently in the buffer.
        self.write_index - self.read_index
    }

    pub fn capacity(&self) -> usize {
        // Return the length of the internal buffer.
        self.buffer.capacity()
    }

    pub fn display_buffer<F>(&mut self, formatter: F)
    where
        F: Fn(&T) -> String,
        {
        println!("Current buffer:");
        let formatted_data: Vec<String> = self.buffer.iter().map(|item| formatter(item)).collect();
        println!("Buffer Data: {:?}", formatted_data);
        println!("Read Index: {}", self.read_index);
        println!("Write Index: {}", self.write_index);
        println!("Buffer Length: {}", self.len());
        println!("Buffer Capacity: {}", self.capacity());
        println!();

    }
}
