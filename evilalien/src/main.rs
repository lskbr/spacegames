/*
Rust initial translation of Evil Alien
Nilo Menezes <nilo@nilo.pro.br>

Isaaman Daniel, and Jenny Tyler
Computer Spacegames
London: Usborne Publishing Ltd, 1982

Found at:
https://usborne.com/browse-books/features/computer-and-coding-books/

5 CLS
10 PRINT "EVIL ALIEN"
20 LET S=10
30 LET G=4
40 LET X=INT(RND*S)
50 LET Y=INT(RND*S)
60 LET D=INT(RND*S)
70 FOR I=1 TO G
80 PRINT "X POSITION (0 TO 9)?"
85 INPUT X1
90 PRINT "Y POSITION (0 TO 9)?"
100 INPUT Y1
110 PRINT "DISTANCE (0 TO 9)?"
120 INPUT D1
130 IF X=X1 AND Y=Y1 AND D=D1 THEN GOTO 300
140 PRINT "SHOT WAS ";
150 IF Y1>Y THEN PRINT "NORTH";
160 IF Y1<Y THEN PRINT "SOUTH";
170 IF X1>X THEN PRINT "EAST";
180 IF X1<X THEN PRINT "WEST";
190 PRINT
200 IF D1>D THEN PRINT "TOO FAR"
210 IF D1<D THEN PRINT "NOT FAR ENOUGH"
220 NEXT I
230 PRINT "YOUR TIME HAS RUN OUT"
240 STOP
300 PRINT "*BOOM* YOU GOT HIM!"
310 STOP

*/
extern crate rand;
extern crate spacegamescommon;

use rand::{thread_rng, Rng};
use spacegamescommon::{cls, read};

fn main() {
    let mut rng = thread_rng();
    cls();
    let s: u8 = 10; // grid size
    let g: u8 = 4; // number of tries
    let x: u8 = rng.gen_range(1, s); // x position
    let y: u8 = rng.gen_range(1, s); // y position
    let d: u8 = rng.gen_range(1, s); // distance
    for _ in 1..g {
        println!("X Position (0 to 9)?");
        let x1: u8 = read();
        println!("Y Position (0 to 9)?");
        let y1: u8 = read();
        println!("D Position (0 to 9)?");
        let d1: u8 = read();
        if x == x1 && y == y1 && d == d1 {
            println!("*BOOM* you got him!");
            return;
        }
        print!("Shot was ");
        if y1 > y {
            print!("north");
        } else if y1 < y {
            print!("south");
        }
        if x1 > x {
            print!("east");
        } else if x1 < x {
            print!("west");
        }
        println!();
        if d1 > d {
            println!("Too far");
        } else if d1 < d {
            println!("Not far enough");
        }
    }
    println!("Your time has run out");
}
