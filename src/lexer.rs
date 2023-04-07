use std::fs;
use crate::token::{Token, Tag};

pub fn scan_file(path: &str) -> Vec<Token> {
    let contents = fs::read_to_string(path);

    let mut tokens: Vec<Token> = vec![];
    match contents {
        Ok(c) => scan(&mut tokens, c.as_str()),
        Err(e) => eprintln!("unable to read file. error: {e:?}"),
    }

    let eof = Token { text: String::new(), tag: Tag::EOF };
    tokens.push(eof);
    tokens
}

pub fn scan(tokens: &mut Vec<Token>, source: &str) {
    let bytes = source.as_bytes();
    let mut start: usize;
    let mut current: usize = 0;
    while current < bytes.len() - 1 {
        start = current;
        current += 1;
        let b = bytes[current];
        let token = match b {
            b'#' => header(bytes, start, &mut current),
            b'*' => {
                if peek(bytes, current) == b' ' {
                    let mut lines: Vec<String> = vec![];
                    unordered_list(bytes, start, &mut current, b'*', &mut lines)
                } else {
                    //bold
                    None
                }
            } 
            b'-' => {
                if peek(bytes, current) == b'-' && peek_next(bytes, current) == b'-' {
                    //hr
                    None
                } else {
                    let mut lines: Vec<String> = vec![];
                    unordered_list(bytes, start, &mut current, b'-', &mut lines)
                }
            }
            b'+' => {
                let mut lines: Vec<String> = vec![];
                unordered_list(bytes, start, &mut current, b'+', &mut lines)
            }
            b'\t' | b' ' | b'\r' | b'\n' => None, 
            _ => paragraph(bytes, start, &mut current),
        };
        
        token.map_or((), |t| tokens.push(t));
    }
}

const fn peek(bytes: &[u8], current: usize) -> u8 {  
    if current < bytes.len() - 1 {
        bytes[current + 1]
    } else {
        b'\0'
    }
}

const fn peek_next(bytes: &[u8], current: usize) -> u8 {
    if current < bytes.len() - 2 {
        bytes[current + 2]
    } else {
        b'\0'
    }
}

fn peek_line(bytes: &[u8], current: usize) -> u8 {
    let mut current_copy = current.clone();
    while bytes[current_copy] != b'\n' {
        current_copy += 1;
    }

    bytes[current]
}

fn paragraph(bytes: &[u8], start: usize, current: &mut usize) -> Option<Token> {
    while peek(bytes, *current) != b'\n' && peek(bytes, *current) != b'\0' {
        *current += 1;
    }

    let p_bytes = &bytes[start..=*current];
    let text = String::from_utf8(p_bytes.to_vec());
    
    text.map_or(None, |t| Some(Token { text: t.trim().to_string(), tag: Tag::P }))
}

fn header(bytes: &[u8], start: usize, current: &mut usize) -> Option<Token> {
    let mut num_of_hashtags = 0;
    while bytes[*current] == b'#' {
        *current += 1;
        num_of_hashtags += 1;
    }

    let next_char_index = start + num_of_hashtags + 1;
    if bytes[next_char_index] != b' ' {
        return paragraph(bytes, start, current);
    }

    *current += 1; 
    while peek(bytes, *current) != b'\n' && peek(bytes, *current) != b'\0'{
        *current += 1;
    }

    let mut end = *current;
    while bytes[end] == b'#' {
        end -= 1;
    }

    if num_of_hashtags <= 6 {
        let header_start = start + num_of_hashtags + 1;
        let header_bytes = &bytes[header_start..=end];
        let text = String::from_utf8(header_bytes.to_vec());

        match text {
            Ok(t) => {
                let header = t
                    .trim()
                    .to_string();
                Some(Token { text: header, tag: Tag::H(num_of_hashtags) })
            }
            Err(_e) => None,
        }
    } else {
        paragraph(bytes, start, current)
    }
}

fn unordered_list(bytes: &[u8], start: usize, current: &mut usize, delimiter: u8, lines: &mut Vec<String>) -> Option<Token> {
    advance_line(bytes, current);

    let text = String::from_utf8(bytes[start + 3..*current].to_vec()).expect("");

    lines.push(text);
    advance_line(bytes, current);

    advance_whitespace(bytes, current);
    let next = String::from_utf8(bytes[*current..].to_vec()); 
    if bytes[*current] == delimiter && peek(bytes, *current) == b' ' {
        unordered_list(bytes, *current, current, delimiter, lines);
        None
    } else {
        let tag = Tag::UL(lines.to_vec());
        Some(Token { text: "".to_string(), tag })
    }
}

fn advance_line(bytes: &[u8], current: &mut usize) { 
    while bytes[*current] != b'\n' {
        *current += 1;
    }
    if *current < bytes.len() - 1 {
        *current += 1;
    }
}

fn advance_whitespace(bytes: &[u8], current: &mut usize) {
    while *current < bytes.len() - 1 && (bytes[*current] == b'\t' || bytes[*current] == b'\r' || bytes[*current] == b' ') {
        *current += 1;
    }
}

#[test]
fn test_paragraph() {
    let source = "this is a paragraph.
        this is a separate one.";

    let expected = vec![
        Token { text: "this is a paragraph.".to_string(),tag: Tag::P },
        Token { text: "this is a separate one.".to_string(), tag: Tag::P },
    ];

    let mut tokens: Vec<Token> = vec![]; 
    scan(&mut tokens, source);

    assert_eq!(tokens, expected);
}
 
#[test]
fn test_header() {
    let source = " 
        # header 1

        ## header 2

        ### header 3

        #### header 4

        ##### header 5

        ###### header 6
        
        ### this is a longer header

        # this is a header with a #hashtag in the middle
        
        # header 1 #

        ## header 2 ##

        ### header 3 ###

        #### header 4 ####

        ##### header 5 #####

        ###### header 6 ######

        ####### not a header

        #not a header
        ";

    let mut tokens: Vec<Token> = vec![]; 
    scan(&mut tokens, source);

    let expected: Vec<Token> = vec![
    Token { text: "header 1".to_string(), tag: Tag::H(1) },
    Token { text: "header 2".to_string(), tag: Tag::H(2) },
    Token { text: "header 3".to_string(), tag: Tag::H(3) },
    Token { text: "header 4".to_string(), tag: Tag::H(4) },
    Token { text: "header 5".to_string(), tag: Tag::H(5) },
    Token { text: "header 6".to_string(), tag: Tag::H(6) },
    Token { text: "this is a longer header".to_string(), tag: Tag::H(3) },
    Token { text: "this is a header with a #hashtag in the middle".to_string(), tag: Tag::H(1) },
    Token { text: "header 1".to_string(), tag: Tag::H(1) },
    Token { text: "header 2".to_string(), tag: Tag::H(2) },
    Token { text: "header 3".to_string(), tag: Tag::H(3) },
    Token { text: "header 4".to_string(), tag: Tag::H(4) },
    Token { text: "header 5".to_string(), tag: Tag::H(5) },
    Token { text: "header 6".to_string(), tag: Tag::H(6) },
    Token { text: "####### not a header".to_string(), tag: Tag::P },
    Token { text: "#not a header".to_string(), tag: Tag::P },
    Token { text: String::new(), tag: Tag::EOF },
    ];

    for (i, t) in tokens.iter().enumerate() {
        assert_eq!(expected[i], *t, "assertion failure on tokens[{i}]");
    }
}

#[test]
fn test_unordered_list() {
    let source = "
    - this a dash list
    - to make a dash list
    - use dashes STOOOPID

    * this a star list
    * to make a star list
    * use stars STOOOPID

    + this a plus list
    + to make a plus list
    + use pluses STOOOPID
    ";

    let expected = vec![
        Token { text: String::new(), 
            tag: Tag::UL(vec![
            "this a dash list".to_string(), 
            "to make a dash list".to_string(),
            "use dashes STOOOPID".to_string(),
        ])},
        Token { text: String::new(), 
            tag: Tag::UL(vec![
            "this a star list".to_string(),
            "to make a star list".to_string(),
            "use stars STOOOPID".to_string(),
        ])},
        Token { text: String::new(), tag: Tag::UL(vec![
            "this a plus list".to_string(),
            "to make a plus list".to_string(),
            "use pluses STOOOPID".to_string(),
        ])},
    ];

    let mut tokens = vec![];
    scan(&mut tokens, source);

    for (i, e) in expected.iter().enumerate() {
        let t = &tokens[i];
        println!("expected: {e:?} actual: {t:?}");
        assert_eq!(e, t);
    }
}
