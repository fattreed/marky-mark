#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token {
    pub text: String,
    pub tag: Tag,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Tag {
    H(usize),
    
    P, A, IMG, 

    UL(Vec<String>), OL(Vec<String>),

    EOF
}

