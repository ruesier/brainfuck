use std::vec::Vec;
use std::fmt;

#[derive(Clone, Copy)]
enum Operation {
    Add, // +
    Subtract, // -
    Read, // ,
    Write, // .
    Left, // <
    Right, // >
    Start, // [
    End // ]
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operation::Add => write!(f, "+"),
            Operation::Subtract => write!(f, "-"),
            Operation::Read => write!(f, ","),
            Operation::Write => write!(f, "."),
            Operation::Left => write!(f, "<"),
            Operation::Right => write!(f, ">"),
            Operation::Start => write!(f, "["),
            Operation::End => write!(f, "]"),
        }
    }
}

use std::num::Wrapping;

struct Tape {
    head: usize,
    data: Vec<Wrapping<u8>>,
}

impl Tape {
    fn new() -> Tape {
        Tape{
            head: 0,
            data: vec![Wrapping(0)],
        }
    }

    fn add(&mut self, delta: &u8) {
        self.data[self.head] += Wrapping(*delta);
    }

    fn subtract(&mut self, delta: &u8) {
        self.data[self.head] -= Wrapping(*delta);
    }

    fn shift_right(&mut self, delta: &usize) {
        while self.data.len() <= self.head + delta {
            self.data.push(Wrapping(0));
        }
        self.head += delta;
    }

    fn shift_left(&mut self, delta: &usize) {
        while self.head < *delta {
            self.data.insert(0, Wrapping(0));
            self.head += 1;
        }
        self.head -= delta;
    }

    fn read(&self) -> u8 {
        self.data[self.head].0.clone()
    }

    fn write(&mut self, val: &u8) {
        self.data[self.head] = Wrapping(*val);
    }
}

impl fmt::Display for Tape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut tape = String::new();
        if self.head == 0 {
            tape.push_str(format!("<{}>", self.data[0]).as_str());
        } else {
            tape.push_str(format!("{}", self.data[0]).as_str());
        }
        for i in 1..self.data.len() {
            if self.head == i {
                tape.push_str(format!(" <{}>", self.data[i]).as_str());
            } else {
                tape.push_str(format!(" {}", self.data[i]).as_str());
            }
        }
        write!(f, "{}", tape)
    }
}

struct Machine {
    tape: Tape,
    operations: Vec<Operation>,
    instruction_pointer: usize,
    loop_stack: Vec<usize>,
    jumping: Option<usize>,
}

impl Machine {
    fn new(operations: Vec<Operation>) -> Machine {
        Machine {
            tape: Tape::new(),
            operations,
            instruction_pointer: 0,
            loop_stack: Vec::new(),
            jumping: None,
        }
    }

    fn step(&mut self, read: &mut dyn io::Read, write: &mut dyn io::Write) -> bool { // returns false when no more operations to perform
        if self.instruction_pointer >= self.operations.len() {
            return false;
        }
        match self.jumping {
            None => match self.operations[self.instruction_pointer] {
                Operation::Add => self.tape.add(&1),
                Operation::Subtract => self.tape.subtract(&1),
                Operation::Read => {
                    let buf: &mut [u8; 1] = &mut [0];
                    read.read_exact(buf).expect(format!("read failed, ip = {}", self.instruction_pointer).as_str());
                    self.tape.write(&(buf[0]));
                },
                Operation::Write => {
                    let buf = &[self.tape.read()];
                    write.write(buf).expect(format!("write failed, ip = {}", self.instruction_pointer).as_str());
                },
                Operation::Left => {
                    self.tape.shift_left(&1);
                },
                Operation::Right => {
                    self.tape.shift_right(&1);
                },
                Operation::Start => {
                    if self.tape.read() == 0 {
                        self.jumping = Some(self.instruction_pointer);
                    }
                    self.loop_stack.push(self.instruction_pointer);
                },
                Operation::End => {
                    let jump = self.loop_stack.pop().unwrap();
                    if self.tape.read() != 0 {
                        self.instruction_pointer = jump;
                        return true;
                    }
                },
            },
            Some(from) => match self.operations[self.instruction_pointer] {
                Operation::Start => {
                    self.loop_stack.push(self.instruction_pointer);
                },
                Operation::End => {
                    let start = self.loop_stack.pop().unwrap();
                    if from == start {
                        self.jumping = None;
                    }
                },
                _ => {},
            },
        };
        self.instruction_pointer += 1;
        self.instruction_pointer < self.operations.len()
    }
}

impl fmt::Display for Machine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ops: Vec<String> = self.operations[self.instruction_pointer..].to_vec().iter().map(|x| format!("{}", x)).collect();
        write!(f, "{}: {}", self.tape, ops.join(""))
    }
}

#[macro_use]
extern crate clap;
use clap::App;

use std::fs::read;
use std::io;

fn main() -> io::Result<()> {
    let yml_config = load_yaml!("cli.yml");
    let matches = App::from_yaml(yml_config).get_matches();

    let script_name = matches.value_of("SCRIPT").unwrap();

    let script = read(script_name)?;

    let mut operations: Vec<Operation> = Vec::new();
    for c in script {
        match c {
            b'+' => {
                operations.push(Operation::Add);
            },
            b'-' => {
                operations.push(Operation::Subtract);
            },
            b',' => {
                operations.push(Operation::Read);
            },
            b'.' => {
                operations.push(Operation::Write);
            },
            b'>' => {
                operations.push(Operation::Right);
            },
            b'<' => {
                operations.push(Operation::Left);
            },
            b'[' => {
                operations.push(Operation::Start);
            },
            b']' => {
                operations.push(Operation::End);
            },
            _ => {},
        }
    }
    let debug = matches.is_present("debug");
    let mut m = Machine::new(operations);
    if debug {
        eprintln!("{:20}", m)
    }
    while m.step(&mut io::stdin(), &mut io::stdout()) {
        if debug {
            eprintln!("{:20}", m);
        }
    }
    println!("\nEND");
    Ok(())
}
