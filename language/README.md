# Language

## Compilation

```bash
$ rustc $PATH_TO_FILE
$ ./$FILE_NAME
```

do not use same file name twice even in different directories, if have to separate by using an underscore

## Cargo

To create a new project `cargo new $PROJECT_NAME`
- pass --vcs flag for no version control system

build executables `cargo build`
build and run `cargo run`
simulate build `cargo check`
build for realease (optimizations) `cargo build --release`
update dependencies `cargo update`
to get docs on dependencies `cargo doc --open`

## Macros vs Functions

Functions:
- Function is called by using ()

Macros:
- A macro is called by using ()!

## Print

```
println!(); # to print and insert new line
print!(); #just print no new line
```

## Functions

`
fn $FUNCTION_NAME() {

}
`

## Variables

Variables are immutable by default. To make them mutable you need to add mut keyword before the variable name.

```
let x = 3; // immutable variable
let mut y = 4; // mutable variable
```