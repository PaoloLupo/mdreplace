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

enum SyntaxKind {
    MarkdownText,
    DoubleCurlyBraces,
}

/// Divide el texto en tokens
struct Lexer<'s> {
    s: Scanner<'s>,
    mode: LexMode,
    error: Option<(EcoString, ErrorPos)>,
}

enum LexMode {
    Markdown,
    Variables,
}

impl<'s> Lexer<'s> {
    fn new(s: &'s str, mode: LexMode) -> Self {
        Self {
            s: Scanner::new(s),
            mode,
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

    fn next(&mut self) -> Option<(SyntaxKind, &'s str)> {
        let (kind, text) = self.s.next()?;
        let kind = match kind {
            unscanny::SyntaxKind::MarkdownText => SyntaxKind::MarkdownText,
            unscanny::SyntaxKind::DoubleCurlyBraces => SyntaxKind::DoubleCurlyBraces,
        };
        Some((kind, text))
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
