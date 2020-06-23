use std::io::{stdin, Read};
use std::process::abort;

#[no_mangle]
pub extern fn write_char(i: i8) {
    print!("{}", i as u8 as char);
}

#[no_mangle]
pub extern fn read_char() -> i8 {
    let mut buf = [0u8];
    if let Err(e) = stdin().read_exact(&mut buf) {
        eprintln!("{}", e);
        abort();
    };

    buf[0] as i8
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
