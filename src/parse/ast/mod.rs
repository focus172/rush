pub enum ASTreeItem {
    Word(String),
    /// Matching [`Token`][`Equals`]. Can be used when assigning a variable or
    /// making an env for a command.
    Assign(String),
    /// `&`
    Background,
    /// `||`
    Or,
    /// `&&`
    And,
    /// `$( *[`ASTreeItem`] )`
    Subshell(Vec<ASTreeItem>),
    /// `# *[`Token`]`
    Comment(String),
}
