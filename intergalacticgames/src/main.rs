/*
Rust initial translation of Intergalactic Games
Nilo Menezes <nilo@nilo.pro.br>

Isaaman Daniel, and Jenny Tyler
Computer Spacegames
London: Usborne Publishing Ltd, 1982

Found at:
https://usborne.com/browse-books/features/computer-and-coding-books/

10 PRINT "INTERGALACTIC GAMES"
20 LET H=INT(RND*100+1)
30 PRINT "YOU MUST LUNCH A SATELITE"
40 PRINT "TO A HEIGHT OF ";H
50 FOR G=1 TO 8
60 PRINT "ENTER ANGLE (0-90)"
70 INPUT A
80 PRINT "ENTER SPEED (0-40000)"
90 INPUT V
100 LET A=A-ATN(H/3)*180/3.14159
110 LET V=V-3000*SQR(H+1/H)
120 IF ABS(A)<2 AND ABS(V)<100 THEN GOTO 200
130 IF A<-2 THEN PRINT "TOO SHALLOW"
140 IF A>2 THEN PRINT "TOO STEEP"
150 IF V<-100 THEN PRINT "TOO SLOW"
160 IF V>100 THEN PRINT "TOO FAST"
170 NEXT G
180 PRINT "YOU'VE FAILED"
190 PRINT "YOU'RE FIRED"
200 STOP
210 PRINT "YOU'VE DONE IT"
220 PRINT "NCTV WINS-THANKS TO YOU"
230 STOP

*/
extern crate rand;
extern crate spacegamescommon;

use rand::{thread_rng, Rng};
use spacegamescommon::read;

fn main() {
    let mut rng = thread_rng();
    println!("Intergalactic Games");
    let h: u16 = rng.gen_range(1, 100);
    println!("You must lunch a satellite");
    println!("to a height of {}", h);
    let fh = f32::from(h);
    for _ in 1..8 {
        println!("Enter angle (0-90)");
        let a: f32 = read();
        println!("Enter speed (0-40000)");
        let v: f32 = read();
        let a = a - ((fh / 3.0).atan() * 180.0 / std::f32::consts::PI);
        let v = v - 3000.0 * (fh + 1.0 / fh).sqrt();
        if a.abs() < 2.0 && v.abs() < 100.0 {
            println!("You've done it");
            println!("NCTV wins - thanks to you");
            return;
        }
        if a < -2.0 {
            println!("Too shallow");
        }
        if a > 2.0 {
            println!("Too steep");
        }
        if v < -100.0 {
            println!("Too slow");
        }
        if v > 100.0 {
            println!("Too fast");
        }
    }
    println!("You've failed");
    println!("You're fired");
}
