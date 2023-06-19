#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum SyntaxKind {
    MarkdownText,
    DoubleCurlyBraces,
    Variables,
    Keywords,
    EOF,
}
