extern crate termion;
extern crate epoll;
extern crate libc;

use std::io;
use std::os::unix::io::AsRawFd;
use termion::raw::IntoRawMode;
use std::str::FromStr;
use std::io::Read;

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


/// Waits for a key press timeout milisseconds
/// 0 - no wait
/// <0 - wait until a key is pressed
/// >0 wait for an event or timeout millisecods for an event.
pub fn inkey(timeout: i32) -> bool {
    let input = io::stdin();
    let _output = io::stdout().into_raw_mode().unwrap();
    let fd = input.as_raw_fd();
    let pool = epoll::create(true).unwrap();
    let e = epoll::Event {
        events: epoll::Events::EPOLLIN.bits(),
        data: 0,
    };
    let _ = epoll::ctl(pool, epoll::ControlOptions::EPOLL_CTL_ADD, fd, e);
    let mut v = [e];
    let wresult = epoll::wait(pool, timeout, &mut v).unwrap();
    unsafe {
        libc::close(pool);
    }
    return wresult == 1;
}

/// Returns a byte read from stdin
pub fn readbyte() -> u8 {
    let mut buffer: [u8; 1] = [0; 1];
    let _ = io::stdin().read(&mut buffer);    
    return buffer[0];
}