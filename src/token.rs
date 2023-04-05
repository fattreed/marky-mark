#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub text: String,
    pub tag: Tag,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Tag {
    H(usize),
    
    P, A, IMG, UL, OL,

    EOF
}
