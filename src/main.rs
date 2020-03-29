use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}


fn main() {
    let result = std::fs::read_to_string("/home/richel/GitHubs/xenon_zero/sprites.txt");
    match result {
        Ok(content) => { println!("File content: {}", content); }
        Err(error) => { println!("Oh noes: {}", error); }
    }

    let content = std::fs::read_to_string("/home/richel/GitHubs/xenon_zero/sprites.txt")
      .expect("could not read file from '/home/richel/GitHubs/xenon_zero/sprites.txt'");

    println!("1111111");
    for line in content.lines() {
        println!("Another line: {}", line);
    }
    println!("Hello, world!!!!");
}
