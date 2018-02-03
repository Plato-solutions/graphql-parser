use std::default::Default;

pub(crate) struct Formatter<'a> {
    buf: String,
    style: &'a Style,
    indent: u32,
}

pub struct Style {
    indent: u32,
}

impl Default for Style {
    fn default() -> Style {
        Style {
            indent: 2,
        }
    }
}

pub(crate) trait Displayable {
    fn display(&self, f: &mut Formatter);
}

impl<'a> Formatter<'a> {
    pub fn new(style: &Style) -> Formatter {
        Formatter {
            buf: String::with_capacity(1024),
            style,
            indent: 0,
        }
    }
    pub fn indent(&mut self) {
        for _ in 0..self.indent {
            self.buf.push(' ');
        }
    }
    pub fn endline(&mut self) {
        self.buf.push('\n');
    }
    pub fn start_block(&mut self) {
        self.indent();
        self.buf.push('{');
        self.endline();
        self.indent += self.style.indent;
    }
    pub fn end_block(&mut self) {
        self.indent = self.indent.checked_sub(self.style.indent)
            .expect("negative indent");
        self.indent();
        self.buf.push('}');
        self.endline();
    }
    pub fn write(&mut self, s: &str) {
        self.buf.push_str(s);
    }
    pub fn into_string(self) -> String {
        self.buf
    }
}