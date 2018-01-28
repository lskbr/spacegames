extern crate rand;
extern crate spacegamescommon;
extern crate termion;

use spacegamescommon::{cls, inkey, readbyte};
use rand::{thread_rng, Rng};
use std::{thread, time};
/*
Rust initial translation of Beat the Bug Eyes
Nilo Menezes <nilo@nilo.pro.br>

Isaaman Daniel, and Jenny Tyler
Computer Spacegames
London: Usborne Publishing Ltd, 1982

Found at:
https://usborne.com/browse-books/features/computer-and-coding-books/

10 PRINT "BUG EYES"
20 LET S=0
30 FOR T=1 TO 10
40 CLS
50 FOR I = 1 TO INT(RND*30+20)
60 NEXT I
70 LET R=INT(RND*4+1)
80 GOSUB 210+30*R
90 PRINT "OO"
100 FOR I=1 TO 20
110 LET R$=INKEY$
120 IF R$<>"" THEN GOTO 140
130 NEXT I
140 IF VAL("0"+R$) <> R THEN GOTO 210
150 S=S+1
160 CLS
170 GOSUB 350
180 PRINT "*"
190 FOR J=1 TO 40
200 NEXT J
210 NEXT T
220 PRINT "YOU BLASTER ";S;"/10 BUGS"
230 STOP
240 LET D=5
250 LET A=1
260 GOTO 350
270 LET D=1
280 LET A=9
290 GOTO 350
300 LET D=5
310 LET A=18
320 GOTO 350
330 LET D=10
340 LET A=7
350 FOR I=1 TO D
360 PRINT
370 NEXT I
380 PRINT TAB(A);
390 RETURN

*/

fn position_enemy(d: u8, a: u8) {
    for _ in 1..d {
        println!();
    }
    let s = " ".repeat((a as usize));
    print!("{}", s);
}

fn main() {
    let mut rng = thread_rng();
    let reaction_time = 2000; // Time it waits for a key press
    let pause = time::Duration::from_secs(2); // Time when a bug is hit
    cls();
    println!("Bug Eyes");
    thread::sleep(pause);
    let mut s = 0;
    let mut a;
    let mut d;
    for _ in 1..10 {
        cls();
        thread::sleep(time::Duration::from_millis(rng.gen_range(20, 50) * 20));
        let r = rng.gen_range(1, 5);
        match r {
            1 => {
                d = 5;
                a = 1
            }
            2 => {
                d = 1;
                a = 9
            }
            3 => {
                d = 5;
                a = 18
            }
            4 => {
                d = 10;
                a = 24;
            }
            _ => panic!("Invalid r"),
        }
        position_enemy(d, a);
        println!("OO");
        if inkey(reaction_time) {
            let key: u8 = readbyte();
            if key > 48 && key - 48 == r {
                s += 1;
                cls();
                position_enemy(d, a);
                println!("**");
                thread::sleep(pause);
            }
        }
    }
    println!("You blasted {0}/10 bugs", s);
}
