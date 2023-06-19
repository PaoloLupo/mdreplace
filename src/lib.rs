pub mod parser;
use crate::parser::SyntaxKind;
use ecow::EcoString;
use pyo3::prelude::*;
use std::fs::File;
use std::io::{Read, Result};
use unscanny::Scanner;

fn open_file(path: &str) -> Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

/// Divide el texto en tokens
#[derive(Clone)]
struct Lexer<'s> {
    s: Scanner<'s>,
    mode: LexMode,
    newline: bool,
    error: Option<(EcoString)>,
}

#[derive(Clone, Copy)]
enum LexMode {
    Markdown, // texto
    Code,     // {{ }}
    Keywords, // {% %}
}

impl<'s> Lexer<'s> {
    fn new(text: &'s str, mode: LexMode) -> Self {
        Self {
            s: Scanner::new(text),
            mode,
            newline: false,
            error: None,
        }
    }

    /// Devuelve el modo actual del lexer
    fn mode(&self) -> LexMode {
        self.mode
    }

    /// Cambia el modo del lexer
    fn set_mode(&mut self, mode: LexMode) {
        self.mode = mode;
    }

    fn keywords(&self, start: usize, c: char) -> SyntaxKind {
        todo!()
    }
}

impl Lexer<'_> {
    fn next(&mut self) -> SyntaxKind {
        self.newline = false;
        self.error = None;
        let start = self.s.cursor();
        match self.s.eat() {
            Some(c) => match self.mode {
                LexMode::Markdown => self.markdown(),
                LexMode::Code => self.code(start, c),
                LexMode::Keywords => self.keywords(start, c),
            },

            None => SyntaxKind::EOF,
        }
    }
}

impl Lexer<'_> {
    fn markdown(&mut self) -> SyntaxKind {
        macro_rules! table {
            ($(|$c:literal)*) => {
                static TABLE: [bool; 128] = {
                    let mut table = [false; 128];
                    $(table[$c as usize] = true;)*
                    table
                };
            }
        }

        table! {
            |'{' | '%' | '\n'
        }
        // match
        loop {
            self.s.eat_until(|c: char| {
                TABLE
                    .get(c as usize)
                    .copied()
                    .unwrap_or_else(|| c.is_whitespace())
            });

            let mut s = self.s;

            match s.eat() {
                Some(' ') if s.at(char::is_alphanumeric) => {}
                Some('{') if !s.at('{') => {}
                _ => break,
            }

            self.s = s;
        }
        self.mode = LexMode::Code;

        SyntaxKind::MarkdownText
    }

    fn code(&mut self, start: usize, c: char) -> SyntaxKind {
        match c {
            '{' => {
                if self.s.eat_if('{') {
                    self.s.eat_while(|c| c != '}' && c != '\n');
                    if self.s.eat_if('}') {
                        SyntaxKind::DoubleCurlyBraces
                    } else {
                        self.error = Some("expected '}}'".into());
                        SyntaxKind::Variables
                    }
                } else {
                    self.s.eat_while(|c| c != '{' && c != '\n');
                    if self.s.eat_if('{') {
                        SyntaxKind::DoubleCurlyBraces
                    } else {
                        self.error = Some("expected '{{'".into());
                        SyntaxKind::Variables
                    }
                }
            }
            '%' => {
                if self.s.eat_if('%') {
                    self.s.eat_while(|c| c != '%' && c != '\n');
                    if self.s.eat_if('%') {
                        SyntaxKind::DoubleCurlyBraces
                    } else {
                        self.error = Some("expected '%%'".into());
                        SyntaxKind::Variables
                    }
                } else {
                    self.s.eat_while(|c| c != '%' && c != '\n');
                    if self.s.eat_if('%') {
                        SyntaxKind::DoubleCurlyBraces
                    } else {
                        self.error = Some("expected '%%'".into());
                        SyntaxKind::Variables
                    }
                }
            }
            '\n' => {
                self.newline = true;
                SyntaxKind::MarkdownText
            }
            _ => {
                self.s.eat_while(|c| c != '{' && c != '%' && c != '\n');
                SyntaxKind::MarkdownText
            }
        }
    }
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn open_file_as_string(path: &str) -> PyResult<String> {
    Ok(open_file(path).unwrap())
}

/// A Python module implemented in Rust.
#[pymodule]
fn mdreplace(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(open_file_as_string, m)?)?;
    Ok(())
}

/// Divide el texto en tokens

fn lex(text: &str) -> Vec<SyntaxKind> {
    let mut lexer = Lexer::new(text, LexMode::Markdown);
    let mut tokens = Vec::new();
    loop {
        let kind = lexer.next();
        tokens.push(kind);
        if kind == SyntaxKind::EOF {
            break;
        }
    }
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lexer() {
        let text = open_file("/home/paolo/dev/rust/mdreplace/mdtest/01.md").unwrap();
        let tokens = lex(&text);
        println!("{:?}", tokens);
    }
}
