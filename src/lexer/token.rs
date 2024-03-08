use crate::prelude::*;

/// Repersents a Token from the input. The input must outlive this value.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Token /* <'a>*/ {
    /// `|`
    Pipe,
    /// `&`
    Amp,
    /// `;`
    SemiColor,
    /// `<`
    LeftArrow,
    /// `>`
    RightArrow,
    /// `(`
    OpenParen,
    /// `)`
    CloseParen,
    /// `$`
    Doller,
    /// ` `text` `
    // BackTick(&'a str),
    BackTick(String),
    /// `\c`
    Escape(char),
    /// `"text"`
    ///
    /// Perserves the literal value of the string with the exception of:
    /// - <dollar-sign>{captures}
    /// - <backquote>{captures}
    /// - <backslash>{captures}
    DoubleQuote(String),
    /// `'<ident>'`
    ///
    /// Perserves the literal value of the string with the exception of:
    /// - <backslash><singlequote>
    /// - <backslash><backslash><singlequote>
    SingleQuote(String),
    /// ` `
    Space,
    /// `\t`
    Tab,
    /// `\n`
    Newline,
    /// `*`
    Glob,
    /// `?`
    Huh,
    /// `[`
    OpenBraket,
    /// `#`
    Pound,
    /// `~`
    Tilde,
    /// `=`
    Equal,
    /// `%`
    Percent,
    /// Anything Else
    Ident(String),
}

#[derive(Debug)]
pub enum TokenError {
    CoerceError(Token),
}
impl fmt::Display for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
impl Context for TokenError {}

impl Token {
    pub fn as_ident(self) -> Result<String, TokenError> {
        match self {
            Token::Ident(s) => Ok(s),
            t => Err(Report::new(TokenError::CoerceError(t))),
        }
    }
}
