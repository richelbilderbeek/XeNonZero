fn create_str_of_spaces(n_chars: u8) -> String {
    let mut s = String::new();
    for number in 0..n_chars {
      s.push(' ');
    }
    return s;
}

fn test() -> () {
    assert!(create_str_of_spaces(0) == "".to_string());
    assert!(create_str_of_spaces(1) == " ".to_string());
    assert!(create_str_of_spaces(2) == "  ".to_string());
    assert!(create_str_of_spaces(3) == "   ".to_string());
}

fn main() {
    test();

    let x = 10;
    assert!(x == 10 , "x wasn't true!");

    let mut s = create_str_of_spaces(10);
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
