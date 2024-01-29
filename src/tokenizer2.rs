use std::{iter::Peekable, str::CharIndices};

struct CharPos {
    ch: char,
    offset: usize,
}

struct Range {
    start: usize,
    end: usize,
}

struct Tokenizer<'input> {
    s: &'input str,
    peak: Peekable<CharIndices<'input>>,
    ch: Option<CharPos>,
    bak_ch: Option<CharPos>,
    str_token: Vec<usize>,
    sym_table: HashMap<String, String>,
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

    pub fn reserve(&mut self) {
        
    }

    pub fn insert_id(&mut self) {

    }

    pub fn insert_const(&mut self) {

    }

    

}
