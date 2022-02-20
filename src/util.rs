pub mod util {
    use std::io;

    pub fn read() -> io::Result<()> {
        let mut buffer = String::new();
        let mut stdin = io::stdin(); // We get `Stdin` here.
        stdin.read_line(&mut buffer)?;
        Ok(())
    }
}