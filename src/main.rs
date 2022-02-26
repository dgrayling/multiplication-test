use std::io::{self, Write};

pub mod util {
    use std::io; 

    pub fn read() -> io::Result<String> {
        let mut buffer = String::new();
        let stdin = io::stdin(); // We get `Stdin` here.
        stdin.read_line(&mut buffer)?;
        buffer.pop();
        Ok(buffer)
    }
}

pub mod equation {
    use rand::Rng;

    pub fn generate_number() -> i32 {
        let mut rng = rand::thread_rng();
        let n1 = rng.gen_range(1..10);
        return n1;
    }

    pub fn generate_equation() -> i32 {
        let mut rng = rand::thread_rng();
        let n1 = rng.gen::<i32>();
        let n2 = rng.gen::<i32>();
        let n3 = n1 * n2;
        return n1;
    }
}

fn main() -> io::Result<()> {

    loop {        
        // let number = equation::generate_equation();
        let n1 = equation::generate_number();
        let s1 = n1.to_string();
        let n2 = equation::generate_number();
        let s2 = n2.to_string();
        println!("What is {} times {}", s1, s2);

        let n3 = n1*n2;
        let s3 = n3.to_string();
        // println!("s3 {}", s3);

        let x = util::read();
        let stdout = io::stdout();
        let mut handle = stdout.lock();

        let y = match x {
            Ok(x) => {
                let l = x.parse::<String>().unwrap();
                let m = l.parse::<i32>();
                match m {
                    Ok(m) => {
                        println!("You entered the number {}", m);
                        if (m == n3) {
                            println!("correct");
                        } else {
                            println!("wrong");
                        }
                    }
                    Err(e) => {
                        println!("{}", e);
                    }
                }
            }
            Err(e) => {
                handle.write_all(b"user error, try again")?;
            }
        };
    }

    Ok(())
}