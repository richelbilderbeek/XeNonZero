
fn create_str_of_spaces(n_chars: u8) -> String {
    let mut s = String::new();
    for _i in 0..n_chars {
      s.push(' ');
    }
    s
}

fn create_starfield(n_rows: usize) -> Vec<String> {
  let mut v: Vec<String> = Vec::new();
  v.resize(n_rows,  ". . . . . ".to_string());
  v
}



fn test() -> () {
    assert!(create_str_of_spaces(0) == "".to_string());
    assert!(create_str_of_spaces(1) == " ".to_string());
    assert!(create_str_of_spaces(2) == "  ".to_string());
    assert!(create_str_of_spaces(3) == "   ".to_string());

    assert!(8 == create_starfield(8).len());
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



    println!("Please enter a key (ASDWX) or 'help'.");

    //let mut key = String::new();
    //std::io::stdin().read_line(&mut key)
    //  .expect("Please enter a key");
    //println!("You pressed key: {}", key);

    println!("Got all the way to the end!");
}
