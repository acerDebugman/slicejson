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
    str_token: Range,
}

impl<'input> Tokenizer<'input> {
    pub fn new(input: &'input str) -> Tokenizer<'input> {
        Self {
            s: input,
            peak: input.char_indices().peekable(),
            ch: None,
            bak_ch: None,
            str_token: Range{ start: 0, end: 0},
        }
    }

    pub fn get_char(&mut self) {
        if self.bak_ch.is_some() {
            self.ch = self.bak_ch.take();
            self.str_token.end = self.ch.as_mut().unwrap().offset;
            self.bak_ch = None;
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

    pub fn retract(&mut self) {
        self.bak_ch = self.ch.take();
    }


}
