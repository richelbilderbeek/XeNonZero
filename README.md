# xenon_zero
Rust version of XeNonZero

## CLI

```
richel@sonic:~/GitHubs/xenon_zero$ ./target/debug/xenon_zero --help
xenon_zero 0.1.0
richelbilderbeek <richel@richelbilderbeek.nl>
Search for a pattern in a file and display the lines that contain it.

USAGE:
    xenon_zero <pattern> <path>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <pattern>    The pattern to look for
    <path>       The path to the file to read

```

Example:

```
richel@sonic:~/GitHubs/xenon_zero$ cargo run -- weligweogd bweugigusbwe
   Compiling xenon_zero v0.1.0 (/home/richel/GitHubs/xenon_zero)
    Finished dev [unoptimized + debuginfo] target(s) in 1.52s
     Running `target/debug/xenon_zero weligweogd bweugigusbwe`
XXXX
A weligweogd
B "bweugigusbwe"
Hello, world!!!!
```

