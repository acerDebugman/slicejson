use std::{iter::Peekable, str::CharIndices};

pub struct CharPos {
    pub ch: char,
    pub offset: usize,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct PosRange {
    pub start: usize,
    pub end: usize, //exclusive
}

pub struct Tokenizer<'input> {
    s: &'input str,
    pub peak: Peekable<CharIndices<'input>>,
    pub ch: Option<CharPos>,
    bak_ch: Option<CharPos>,
    pub str_token: Vec<usize>,
}

impl<'input> Tokenizer<'input> {
    pub fn new(input: &'input str) -> Tokenizer<'input> {
        Self {
            s: input,
            peak: input.char_indices().peekable(),
            ch: None,
            bak_ch: None,
            str_token: vec![],
        }
    }

    pub fn get_char(&mut self) {
        if self.bak_ch.is_some() {
            self.ch = self.bak_ch.take();
            return;
        }
        let next = self.peak.next();
        match next {
            None => self.ch = None,
            Some((offset, ch)) => {
                self.ch.as_mut().unwrap().ch = ch;
                self.ch.as_mut().unwrap().offset = offset;
            }
        }
    }

    pub fn concat(&mut self) {
        let ch = self.ch.take();
        self.str_token.push(ch.unwrap().offset);
    }

    pub fn skip_bc(&mut self) {
        while self.ch.as_ref().unwrap().ch == '\u{20}'
            || self.ch.as_ref().unwrap().ch == '\n'
            || self.ch.as_ref().unwrap().ch == '\r'
            || self.ch.as_ref().unwrap().ch == '\t'
        {
            self.get_char();
        }
    }

    pub fn is_digit(&self) -> bool {
        if '0' <= self.ch.as_ref().unwrap().ch && self.ch.as_ref().unwrap().ch <= '9' {
            return true;
        }
        return false;
    }

    pub fn is_letter(&self) -> bool {
        if ('a' <= self.ch.as_ref().unwrap().ch && self.ch.as_ref().unwrap().ch <= 'z')
            || ('A' <= self.ch.as_ref().unwrap().ch && self.ch.as_ref().unwrap().ch <= 'Z')
        {
            return true;
        }
        return false;
    }

    //only support once now
    pub fn retract(&mut self) {
        self.bak_ch = self.ch.take();
        self.str_token.pop();
    }

    pub fn reserve(&mut self) {}

    pub fn insert_id(&mut self) {}

    pub fn insert_const(&mut self) {}
}
