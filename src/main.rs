/* 
** Brainfuck Interpreter in Rust
** - Starts at cell 0
** - Both cell and cell value cannot be below 0
** - Max number of cell and cell value varies per machine
*/ 

use std::fs::File;
use std::io::prelude::*;

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

fn get_input() -> usize {
	use std::io;
	let mut i = String::new();
	io::stdin().read_line(&mut i).expect("Failed to get user input!");
	match i.chars().next() {
		Some(c) => c as usize,
		None => panic!("Recived invalid input"),
	}
}

fn get_close(index: usize, bf: &Vec<Command>) -> usize {
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

fn get_open(index: usize, bf: &Vec<Command>) -> usize {
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
	let mut current_cell: usize = 0;
	let mut current_command: usize = 0;
	let bf = get_bf("hello.bf");
 	
	while current_command < bf.len() {
		if let Some(c) = bf.get(current_command) {
			match c {
				Command::Increment => {
					cells[current_cell] += 1;
				},
				Command::Decrement => {
					if cells[current_cell] > 0 {
						cells[current_cell] -= 1;
					} else {
						panic!("Value of cell cannot go below 0");
					}
				},
				Command::ShiftRight => {
					if current_cell == cells.len() -1 {
						cells.push(0);
					}
					current_cell += 1;
				},
				Command::ShiftLeft => {
					if current_cell > 0 {
						current_cell -= 1;
					} else {
						panic!("Cell index cannot be a negative number");
					}
				},
				Command::ForwardJump => {
					if cells[current_cell] == 0 {
						current_command = get_close(current_command, &bf);
					}
				},
				Command::BackwardJump => {
					if cells[current_cell] != 0 {
						current_command = get_open(current_command, &bf);
					}
				},
				Command::Output => {
					if cells[current_cell] < 128 {
						print!("{}", cells[current_cell] as u8 as char);
					}
				},
				Command::Input => {
					cells[current_cell] = get_input();
				},
			}
		}
		current_command += 1;
	}
}
