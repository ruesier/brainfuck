enum Operation {
    Add, // +
    Subtract, // -
    Read, // ,
    Write, // .
    Left, // >
    Right, // <
    Start(u32), // [
    End(u32) // ]
}

struct Tape {
    head: u32,
    data: Vec<i8>,
}

impl Tape {
    fn new() -> Tape {
        Tape{
            head: 0,
            data: Vec<u8>::new(),
        }
    }

    fn add(&mut self, delta: i8) {
        self.data[self.head] += delta;
    }

    fn move(&mut self, delta: i8) {
        while self.data.len() <= self.head + delta {
            self.data.push(0);
        }
        while self.head < -delta {
            self.data.insert(0, 0);
            self.head++;
        }
        self.head += delta;
    }

    fn get(&self) -> &u8 {
        self.data[self.head]
    }

    fn set(&mut self, &u8)
}

fn main() {
    println!("Hello, world!");
}
