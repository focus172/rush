/// Repersents a Token from the input. The input must outlive this value.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Token {
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
    /// '`'
    BackTick,
    /// `\c`
    Escape(char),
    /// `"text"`
    ///
    /// Perserves the literal value of the string with the exception of:
    /// - <dollar-sign>{captures}
    /// - <backquote>{captures}
    /// - <backslash>{captures}
    DoubleQuote,
    /// `'<ident>'`
    ///
    /// Perserves the literal value of the string with the exception of:
    /// - <backslash><singlequote>
    /// - <backslash><backslash><singlequote>
    SingleQuote,
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
    /// `{`
    OpenBraket,
    /// `}`
    CloseBraket,
    /// `#`
    Pound,
    /// `~`
    Tilde,
    /// `=`
    Equal,
    /// `%`
    Percent,
    /// Anything Else
    // Ident(&'a str),
    Ident(String),
}
