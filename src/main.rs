use std::io::{ BufRead, BufReader};
use std::env;
use std::fs::File;

// enum IntrinsicTypes {
// 	HALT,NOP
// }

// struct Intrinsic {
// 	token_type:IntrinsicTypes,
// 	literal:u8,
// 	line:u32,
// 	col:u32
// }

fn main() {
  let args:Vec<_> = env::args().collect();
  // TODO: let mut has_errors = false;

	let args_len = args.len();
	if args_len > 1 {
		if args_len == 2 {
			let file_path = &args[1];
			scan_code_file(file_path);
		}else{
			panic!("TODO: read args from user and interpret");
		}
  }else{
    panic!("TODO: set up a shell");
  }
}

fn scan_code_file(file_path:&str) -> u8 {
	let mut f = BufReader::new(File::open(file_path).expect("ERROR: File not Found"));

	let mut buf = Vec::<u8>::new();
	let mut col_number : u32 = 1;
	let mut line_number : u32 = 1;
	while f.read_until(b'\n', &mut buf).expect("read_until failed") != 0 {
		let s = String::from_utf8(buf).expect("from_utf8 failed");
		let mut word_buffer = String::new();
		for c in s.chars() {
			// end of the line
			if col_number == s.len() as u32{
				word_buffer.push(c);
				if word_buffer.len() > 0 {
					println!("word: {},line: {},col: {}",word_buffer.trim_end(),line_number,col_number-(word_buffer.len() as u32)+1);
				}
				word_buffer.clear();
			}
			else if  c == ' ' || c == '\t' || c=='\n'{
				if word_buffer.len() > 0 {
					println!("word: {},line: {},col: {}",word_buffer,line_number,col_number-(word_buffer.len() as u32));
				}
				word_buffer.clear();
			}
			else{
				word_buffer.push(c);
			}
			col_number +=1;
		}
		line_number += 1;
		col_number = 1;
		// this returns the ownership of the read data to buf
		buf = s.into_bytes();
		buf.clear();
	}
	0
}
