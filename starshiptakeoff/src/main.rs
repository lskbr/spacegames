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
extern crate spacegamescommon;

use rand::{thread_rng, Rng};
use spacegamescommon::{cls, read};

fn main() {
    let mut rng = thread_rng();
    cls();
    println!("Startship take-off");
    let g: u16 = rng.gen_range(1, 20); // Gravity
    let w: u16 = rng.gen_range(1, 40); // Weight (mass)
    let r: u16 = g * w; // Weight
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
