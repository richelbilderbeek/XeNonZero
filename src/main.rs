struct Game {
  n_rows: usize,
  n_cols: usize,
  player_x: i8
}

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
    assert_eq!(create_str_of_spaces(1), " ".to_string());
    assert_eq!(create_str_of_spaces(2), "  ".to_string());
    assert_eq!(create_str_of_spaces(3), "   ".to_string());

    assert_eq!(8, create_starfield(8, 60).len());
    assert_eq!(60, create_starfield(8, 60)[0].len());
}

fn main() {
    test();

    let game = Game { n_cols: 20, n_rows: 10, player_x: 10 };

    println!("........................................");
    let starfield = create_starfield(game.n_rows, game.n_cols);
    for line in starfield {
      println!("{}", line);
    }

    println!("........................................");
    let mut s = create_str_of_spaces(game.player_x);
    s.push('X');
    println!("{}", s);
    println!("........................................");



    println!("Please enter a key (ASDWX) or 'help'.");

    //let mut key = String::new();
    //std::io::stdin().read_line(&mut key)
    //  .expect("Please enter a key");
    //println!("You pressed key: {}", key);

    println!("Got all the way to the end!");
}
