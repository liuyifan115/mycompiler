use std::{fs::File, io::BufRead, path::Path};

pub fn read_source<P>(file_path: P) -> Vec<(String, usize)>
where P: AsRef<Path> {
    let mut lines_with_numbers: Vec<(String, usize)> = Vec::new();
    let source_file = File::open(file_path).unwrap();

    let lines = std::io::BufReader::new(source_file).lines();
    for (index, line) in lines.enumerate() {
        if let Ok(ip) = line {
            lines_with_numbers.push((ip.trim().to_string(), index + 1));
        }
    }
    lines_with_numbers
}

pub fn preprocesser(lines: &Vec<(String, usize)>) -> Vec<(String, usize)> {
    remove_comment(lines)
}

fn remove_comment(lines: &Vec<(String, usize)>) -> Vec<(String, usize)> {
    let mut new_lines: Vec<(String, usize)> = Vec::new();

    #[derive(Debug, PartialEq)]
    enum State {
        Init,
        SingleLineComment,
        MultiLineComment
    }
    let mut state = State::Init;
    for (line, number) in lines.iter() {
        let mut new_line = String::new();
        let mut ch_iter = line.chars().peekable();
        while let Some(mut ch) = ch_iter.next() {
            match state {
                State::Init => {
                    if ch == '/' && ch_iter.peek() == Some(&'/') {
                        state = State::SingleLineComment;
                    } else if ch == '/' && ch_iter.peek() == Some(&'*') {
                        // 避免/*/这样的情况
                        ch_iter.next();

                        state = State::MultiLineComment;
                    } else {
                        new_line.push(ch);
                    }
                }
                State::SingleLineComment => {
                    continue;
                }
                State::MultiLineComment => {
                    if ch == '*' && ch_iter.peek() == Some(&'/') {
                        ch_iter.next();
                        state = State::Init;
                    } else {
                        continue;
                    }
                }
            }
        }
        if state == State::SingleLineComment {
            state = State::Init;
        }
        if !new_line.is_empty() {
            new_lines.push((new_line.trim().to_string(), *number));
        }
    }

    new_lines
}