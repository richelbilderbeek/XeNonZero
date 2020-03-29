use std::io;

fn old_main() {
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

    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}

fn main() {

    let mut x = 10;
    assert!(x == 10 , "x wasn't true!");

    let mut s = String::new();
    for number in 0..x {
      s.push(' ');
    }
    s.push('X');
    println!("........................................");
    println!("{}", s);
    println!("........................................");



    //println!("Please input your guess.");

    //let mut guess = String::new();

    //io::stdin().read_line(&mut guess)
    //    .expect("Failed to read line");

    //println!("You guessed: {}", guess);
    println!("Got all the way to the end!");
}
