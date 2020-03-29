
fn create_str_of_spaces(n_chars: usize) -> String {
    let mut s = String::new();
    for _i in 0..n_chars {
      s.push(' ');
    }
    s
}

fn create_starfield(n_rows: usize, c_cols: usize) -> Vec<String> {
  let mut v: Vec<String> = Vec::new();
  let s = create_str_of_spaces(c_cols);
  v.resize(n_rows, s);
  v
}



fn test() -> () {
    assert_eq!(create_str_of_spaces(0), "".to_string());
    assert!(create_str_of_spaces(1) == " ".to_string());
    assert!(create_str_of_spaces(2) == "  ".to_string());
    assert!(create_str_of_spaces(3) == "   ".to_string());

    assert_eq!(8, create_starfield(8, 60).len());
    assert!(60 == create_starfield(8, 60)[0].len());
}

fn main() {
    test();

    let x = 10;
    assert!(x == 10 , "x wasn't true!");

    let mut s = create_str_of_spaces(10);
    s.push('X');
    println!("........................................");
    let starfield = create_starfield(10, 20);
    for line in starfield {
      println!("{}", line);
    }

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
