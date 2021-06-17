mod utils;
mod structs;

use structs::{Token, Position};
use structs::TokenType;

use utils::{is_lowercase, is_uppercase};

pub fn lex(my_file: String) -> Vec<Token> {
    let mut tokens = Vec::<Token>::new();

    let symbols: Vec<_> = my_file.chars().collect();

    let mut cur_line = 1;
    let mut cur_row = 1;

    let mut i: usize = 0;

    loop {
        while i < symbols.len() && (symbols[i] == ' ' || symbols[i] == '\n') {
            if symbols[i] == ' ' {
                i += 1;
                cur_row += 1;
            } else {
                cur_row = 1;
                cur_line += 1;
                i += 1;
            }
        }

        while i < symbols.len() && (symbols[i] != ' ' && symbols[i] != '\n') {
            if symbols[i] == '+' {
                let token = Token {tt: TokenType::OperatorPlus, pos: Position {line: cur_line, row: cur_row}};
                tokens.push(token);

                i += 1;
                cur_row += 1;

                continue;
            } else if symbols[i] == '-' {
                if i < symbols.len() - 1 {
                    if symbols[i + 1] == '>' {

                        let token = Token {tt: TokenType::OperatorArrow, pos: Position {line: cur_line, row: cur_row}};
                        tokens.push(token);

                        i += 2;
                        cur_row += 2;

                        continue;
                    } else {
                        let temp_str = String::from(symbols[i]);
                        let token = Token {tt: TokenType::Error(temp_str), pos: Position {line: cur_line, row: cur_row}};
                        tokens.push(token);

                        i += 1;
                        cur_row += 1;

                        continue;
                    }
                } else {
                    let temp_str = String::from(symbols[i]);
                    let token = Token {tt: TokenType::Error(temp_str), pos: Position {line: cur_line, row: cur_row}};
                    tokens.push(token);

                    i += 1;
                    cur_row += 1;

                    continue;
                }
            } else if symbols[i].is_digit(10) {
                let mut cur_str = String::new();
                while i < symbols.len() && symbols[i].is_digit(10) {
                    cur_str.push_str(&String::from(symbols[i]));
                    i += 1;
                }

                let token = Token {tt: TokenType::Number(cur_str.clone()), pos: Position {line: cur_line, row: cur_row}};
                tokens.push(token);

                cur_row += cur_str.len() as i32;

                continue;
            } else if is_uppercase(symbols[i]) {
                let mut cur_str = String::new();
                let mut last_sym: char = symbols[i];
                let mut temp_flag: bool = false;

                while i < symbols.len() && (is_uppercase(symbols[i]) || is_lowercase(symbols[i]) || symbols[i].is_digit(10)) {
                    if is_uppercase(symbols[i]) {
                        cur_str.push_str(&String::from(symbols[i]));
                        last_sym = symbols[i];
                        i += 1;
                        continue;
                    }
                    if is_lowercase(symbols[i]) && !last_sym.is_digit(10) {
                        cur_str.push_str(&String::from(symbols[i]));
                        last_sym = symbols[i];
                        i += 1;
                        continue;
                    } else if last_sym.is_digit(10) {
                        last_sym = symbols[i];
                        temp_flag = true;
                        i += 1;
                        break;
                    }
                    if symbols[i].is_digit(10) {
                        cur_str.push_str(&String::from(symbols[i]));
                        last_sym = symbols[i];
                        i += 1;
                        continue;
                    }
                }

                if temp_flag {
                    let token = Token {tt: TokenType::Chem(cur_str.clone()), pos: Position {line: cur_line, row: cur_row}};
                    tokens.push(token);

                    cur_row += cur_str.len() as i32;

                    let token = Token {tt: TokenType::Error(String::from(last_sym)), pos: Position {line: cur_line, row: cur_row}};
                    tokens.push(token);

                    cur_row += 1;
                } else {
                    let token = Token {tt: TokenType::Chem(cur_str.clone()), pos: Position {line: cur_line, row: cur_row}};
                    tokens.push(token);

                    cur_row += cur_str.len() as i32;
                }
                continue;
            } else {
                let token = Token {tt: TokenType::Error(String::from(symbols[i])), pos: Position {line: cur_line, row: cur_row}};
                tokens.push(token);

                cur_row += 1;
                i += 1;

                continue;
            }
        }

        if i >= symbols.len() {
            break;
        }
    }

    tokens
}

