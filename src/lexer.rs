use std::io::{ BufRead, BufReader};
use std::fs::File;

// enum TokenType {
// 	STRING,PRINT
// }

#[derive(Debug)]
#[derive(PartialEq)]
enum LiteralToken{
	STRING,CHAR,NONE
}

#[derive(Debug)]
#[derive(PartialEq)]
enum Keyword{
	PRINT,
}

// struct Token{
// 	token_type: TokenType,
// 	value: String,
// }

pub fn scan_code_file(file_path:&str) {
	let mut f = BufReader::new(File::open(file_path).expect("ERROR: File not Found"));

	let mut buf = Vec::<u8>::new();
	let mut col_number : u32 = 1;
	let mut line_number : u32 = 1;
	while f.read_until(b'\n', &mut buf).expect("read_until failed") != 0 {
		let s = String::from_utf8(buf).expect("from_utf8 failed");

		let mut buffer = String::new();
		let mut literal_token_buffer = LiteralToken::NONE;

		for c in s.chars() {
			if c == '\n'{
				if literal_token_buffer != LiteralToken::NONE {
					panic!("ERROR: Unterminated literal token");
				}
				if buffer.len() > 0 {
					println!("{} -> {}:{}:{}" , buffer,file_path,line_number,col_number - buffer.len() as u32);
				}
				buffer.clear();
			} else if  c == ' ' || c == '\t' || c == '\n' || c == '\r' {
				if literal_token_buffer != LiteralToken::NONE {
					buffer.push(c);
				}else{
					if buffer.len() > 0 {
						println!("{} -> {}:{}:{}" , buffer,file_path,line_number,col_number - buffer.len() as u32);
					}
					buffer.clear();
				}
			} else if c == '"'  {
				if literal_token_buffer == LiteralToken::NONE {
					literal_token_buffer = LiteralToken::STRING;
					// ALERT
					buffer.push(c);
				}else if literal_token_buffer == LiteralToken::STRING {
					literal_token_buffer = LiteralToken::NONE;
					// ALERT
					buffer.push(c);
					// col number is not incremented after the closing quote pushed to buffer
					println!("{} -> {}:{}:{}" , buffer,file_path,line_number,col_number + 1 - buffer.len() as u32);
					buffer.clear();
				}else{
					buffer.push(c);
				}
			} else if c == '\''{
				if literal_token_buffer == LiteralToken::NONE {
					// starting ['] is also included in the literal token
					buffer.push(c);
					literal_token_buffer = LiteralToken::CHAR;
				}else if literal_token_buffer == LiteralToken::CHAR {
					literal_token_buffer = LiteralToken::NONE;
					// eding ['] is also included in the literal token
					buffer.push(c);
					// col number is not incremented after the closing quote pushed to buffer
					println!("{} -> {}:{}:{}" , buffer,file_path,line_number,col_number + 1 - buffer.len() as u32);
					buffer.clear();
				}else{
					buffer.push(c);
				}
			}else{
				buffer.push(c);
			}
			col_number +=1;
		}
		if buffer.len() > 0 {
			println!("{} -> {}:{}:{}" , buffer,file_path,line_number,col_number - buffer.len() as u32);
		}
		buffer.clear();
		line_number += 1;
		col_number = 1;
		// this returns the ownership of the read data to buf
		buf = s.into_bytes();
		buf.clear();
	}
}
