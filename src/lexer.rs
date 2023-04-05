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
    let mut bytes = source.as_bytes();
    let mut start: usize = 0;
    let mut current: usize = 0;
    while current < bytes.len() {
        start = current;
        let prev = current;
        let b = bytes[prev];
        current += 1;

        let token = match b {
            b'#' => header(b, bytes, start, &mut current),
            b'\t' | b' ' | b'\r'| b'\n' => None,
            _ => None,
        };
        
        match token {
            Some(t) => tokens.push(t),
            None => (),
        }
    }
}

fn peek(bytes: &[u8], current: usize) -> u8 {  
    if current <= bytes.len() {
        bytes[current + 1]
    } else {
        b'\0'
    }
}

fn header(b: u8, bytes: &[u8], start: usize, current: &mut usize) -> Option<Token> {
    let mut num_of_hashtags = 1;
    while bytes[*current] == b'#' {
        *current += 1;
        num_of_hashtags += 1;
    }

    *current += 1; 
    while peek(bytes, *current) != b'\n' {
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
            Err(e) => None,
        }
    } else {
        Some(Token { text: "".to_string(), tag: Tag::P })
    }
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
    Token { text: "#######not a header".to_string(), tag: Tag::P },
    Token { text: "#not a header".to_string(), tag: Tag::P },
    Token { text: String::new(), tag: Tag::EOF },
    ];

    for (i, t) in tokens.iter().enumerate() {
        let e = &expected[i];
        assert_eq!(expected[i], *t);
    }
}
