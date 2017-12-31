/*
Rust initial translation of Starship Takeoff
Nilo Menezes <nilo@nilo.pro.br>

Isaaman Daniel, and Jenny Tyler
Computer Spacegames
London: Usborne Publishing Ltd, 1982

Starship takeoff by Richard Nash


Found at:
https://usborne.com/browse-books/features/computer-and-coding-books/

10 CLS
20 PRINT "STARTSHIP TAKE-OFF"
30 LET G=INT(RND*20+1)
40 LET W=INT(RND*40+1)
50 LET R=G*W
60 PRINT "GRAVITY= ";G
70 PRINT "TYPE IN FORCE"
80 FOR C=1 TO 10
90 INPUT F
100 IF F>R THEN PRINT "TOO HIGH"
110 IF F<R THEN PRINT "TOO LOW"
120 IF F=R THEN GOTO 190
130 IF C<>10 THEN PRINT ", TRY AGAIN"
140 NEXT C
150 PRINT
160 PRINT "YOU FAILED -"
170 PRINT "THE ALIENS GOT YOU"
180 STOP
190 PRINT "GOOD TAKE OFF" 
*/
extern crate rand;
extern crate termion;

use std::io;
use std::str::FromStr;
use rand::{thread_rng, Rng};

/// Loop until a valid number is read
fn read<T>() -> T where T: FromStr{
    let mut line = String::new();
    loop {
        io::stdin()
            .read_line(&mut line)
            .expect("failed to read from stdin");
        let no_spaces = line.trim();
        
        match no_spaces.parse::<T>(){
            Ok(v) => return v,
            Err(_) => println!("Invalid value: {}", no_spaces)
        }
    }    
}

/// Clear the screen
fn cls() {
    println!("{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1));
}


fn main() {
    let mut rng = thread_rng();    
    cls();
    println!("Startship take-off");
    let g: u16 = rng.gen_range(1, 20); // Gravity
    let w: u16 = rng.gen_range(1, 40); // Weight (mass)
    let r: u16 = g * w;  // Weight
    println!("Gravity {}", g);
    println!("Type in force");
    for c in 1..10 {
        let f: u16 = read();
        if f > r {
            print!("Too high");
        } else if f < r {
            print!("Too low");
        } else {
            println!("Good take off");
            return;
        }
        if c != 10 {
            println!(", try again")            
        }
    }
    println!("\nYou failed -");
    println!("The aliens got you");
}
