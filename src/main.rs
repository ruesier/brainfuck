use std::vec::*;

enum Operation {
    Add, // +
    Subtract, // -
    Read, // ,
    Write, // .
    Left, // >
    Right, // <
    Start(usize), // [
    End(usize) // ]
}

struct Tape {
    head: usize,
    data: Vec<i8>,
}

impl Tape {
    fn new() -> Tape {
        Tape{
            head: 0,
            data: Vec::new(),
        }
    }

    fn add(&mut self, delta: &i8) {
        self.data[self.head] += *delta;
    }

    fn shift_right(&mut self, delta: &usize) {
        while self.data.len() <= self.head + delta {
            self.data.push(0);
        }
        self.head += delta;
    }

    fn shift_left(&mut self, delta: &usize) {
        while self.head < *delta {
            self.data.insert(0, 0);
            self.head += 1;
        }
        self.head -= delta;
    }

    fn read(&self) -> i8 {
        self.data[self.head].clone()
    }

    fn write(&mut self, val: &i8) {
        self.data[self.head] = *val;
    }
}

fn main() {
    println!("Hello, world!");
}
