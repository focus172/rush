pub enum ASTreeItem {
    Word(Vec<Expand>),
    /// Matching [`Equals`]. Can be used when assigning a variable or
    /// making an env for a command.
    Assign,
    /// `&`
    Background,
    /// `||`
    Or,
    /// `&&`
    And,
    /// `|`
    Pipe,
    /// `$( *[`ASTreeItem`] )`
    Subshell(Vec<ASTreeItem>),
    /// `# *[`Token`]`
    Comment(String),
}

pub enum Expand {}
