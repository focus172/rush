# Rush
Adapted from a project of the same name by `ashpil@pm.me`

## Pipeline
The shell takes a parser and the parser takes a lexer, the lexer takes a data
steam. Each component can be swapped for maximum reuse.

## Iterator
Everything is made using iterators meaning that hypothetically a file that
contains `echo hello world` followed by 100,000,000 lines of comments will:
    - not have memory problems
    - will print 'hello' before the file finishes reading

## To Do
- [X] Simple command execution `ls -ltr`
- [X] Pipes `exa | grep cargo`
- [ ] Exit status logic `! false && ls || date`
- [ ] Redirection
    - [ ] File descriptor to another `ls error 2>&1`
    - [ ] To/from file `date > time.txt` `< Cargo.toml wc`
    - [ ] Appending `>>`
    - [ ] Here-docs `<<`
    - [ ] Raw, non-io file descriptors `4>&7`
- [ ] Async execution `&`
- [ ] Shell builtins
   - [ ] Normal built-ins
      - [ ] `alias` `unalias`
      - [ ] `cd`
      - [ ] etc
   - [ ] Special built-ins
      - [ ] `exit`
      - [ ] `export`
      - [ ] `exec`
      - [ ] etc
- [ ] Expansions
   - [ ] Tilde expansion `ls ~`
   - [ ] Parameter expansion
      - [ ] Basic expansion `echo ${var:-other}`
      - [ ] String length `echo ${#var}`
      - [ ] Suffix/prefix removal `echo ${var%%pattern}`
   - [ ] Command substitution
   - [ ] Arithmetic expansion
- [ ] Variables
- [ ] Quotes
- [ ] IFS
- [ ] Functions
- [ ] Control flow `if` `for` `while` `case` etc
- [ ] Expand this to-do list
