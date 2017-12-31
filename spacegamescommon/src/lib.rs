extern crate termion;

use std::str::FromStr;
use std::io;

/// Loop until a valid number is read
pub fn read<T>() -> T
where
    T: FromStr,
{
    let mut line = String::new();
    loop {
        io::stdin()
            .read_line(&mut line)
            .expect("failed to read from stdin");
        let no_spaces = line.trim();

        match no_spaces.parse::<T>() {
            Ok(v) => return v,
            Err(_) => println!("Invalid value: {}", no_spaces),
        }
    }
}

/// Clear the screen
pub fn cls() {
    println!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
}
