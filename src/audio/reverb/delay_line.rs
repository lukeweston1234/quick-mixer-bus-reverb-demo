pub struct DelayLine {
    buffer: Vec<f32>,
    position: usize
}

impl DelayLine {
    pub fn new(length: usize) -> Self {
        Self {
            buffer: vec![0.0; length],
            position: 0
        }
    }
    pub fn read(&self) -> f32 {
        self.buffer[self.position]
    }
    pub fn write_and_advance(&mut self, value: f32){
        self.buffer[self.position] = value;
        if self.position == self.buffer.len() - 1{
            self.position = 0;
        }
        else {
            self.position += 1;
        }
    }

}