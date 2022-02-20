use std::io::{self, Write};

pub mod util {
    use std::io;

    pub fn read() -> io::Result<String> {
        let mut buffer = String::new();
        let stdin = io::stdin(); // We get `Stdin` here.
        stdin.read_line(&mut buffer)?;
        Ok(buffer)
    }
}

fn main() -> io::Result<()> {
    let x = util::read();

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    // handle.write_all(b"hello world")?;
    
    let y = match x {
        Ok(x) => x,
        Err(e) => return Err(e)
    };

    handle.write(y.as_bytes());

    Ok(())
}