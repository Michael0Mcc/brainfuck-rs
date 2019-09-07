use std::fs::File;
use std::io::prelude::*;
use std::io::ErrorKind;

enum Command {
	Increment,
	Decrement,
	ShiftRight,
	ShiftLeft,
	ForwardJump,
	BackwardJump,
	Output,
	Input,
}

fn get_close(index: usize, bf: Vec<Command>) -> usize {
	let mut stack: Vec<Command> = Vec::new();
	for i in index..bf.len() {
		match bf.get(i) {
			Some(c) => {
				match c {
					Command::ForwardJump => { stack.push(Command::ForwardJump); },
					Command::BackwardJump => { stack.pop(); },
					_ => {},
				}
			},
			None => panic!("Could not find command"),
		}

		if stack.is_empty() {
			return i;
		}
	}
	panic!("'[' at index {} has no close!", index);
}

fn get_open(index: usize, bf: Vec<Command>) -> usize {
	let mut stack: Vec<Command> = Vec::new();
	for i in (0..index+1).rev() {
		match bf.get(i) {
			Some(c) => {
				match c {
					Command::BackwardJump => { stack.push(Command::BackwardJump); },
					Command::ForwardJump => { stack.pop(); },
					_ => {},
				}
			},
			None => panic!("Could not find command"),
		}

		if stack.is_empty() {
			return i;
		}
	}
	panic!("']' at index {} has no open!", index);
}

fn get_bf(path: &str) -> Vec<Command> {
	if !path.ends_with(".bf") {
		panic!("File must have '.bf' extension");
	}

	let mut s = String::new();
	File::open(path).expect("Something went wrong with opening the file!").read_to_string(&mut s).unwrap();

	let mut bf: Vec<Command> = Vec::new();
	for c in s.chars() {
		match c {
			'+' => bf.push(Command::Increment),
			'-' => bf.push(Command::Decrement),
			'>' => bf.push(Command::ShiftRight),
			'<' => bf.push(Command::ShiftLeft),
			'[' => bf.push(Command::ForwardJump),
			']' => bf.push(Command::BackwardJump),
			'.' => bf.push(Command::Output),
			',' => bf.push(Command::Input),
			_ => (),
		}
	}

	bf
}

fn main() {
	let mut cells: Vec<usize> = vec![0];
	let current_cell: usize = 0;
	let current_command: usize = 0;
	let bf = get_bf("hello.bf");
 	println!("{}", get_open(8, bf));
}
