



/// Makes a interruption in console and writes "Press any key to continue"
///# Example
/// ```
/// fn main() {
///     println!("hello, world!");
///     termpause::pause();
/// }
/// ```
/// 
///
pub mod termpause {
use std::io;
use std::io::prelude::*;
    pub fn pause() {
        let mut stdin = io::stdin();
        let mut stdout = io::stdout();
        write!(stdout, "Press any key to continue...").unwrap();
        stdout.flush().unwrap();
    
        let _ = stdin.read(&mut [0u8]).unwrap();
    }

}

