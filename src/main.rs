

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
        let n1 = rng.gen_range(2..10);
        return n1;
    }
}

struct Puzzle {
    s1: String,
    s2: String,
    n3: i32
}

fn generate_puzzle() -> Puzzle {
    let n1 = equation::generate_number();
    let n2 = equation::generate_number();
    let s1 = n1.to_string();
    let s2 = n2.to_string();
    let n3 = n1*n2;
    return Puzzle {
        s1, s2, n3
    }
}

fn main() -> () {

    loop {
        let puzzle = generate_puzzle();

        println!("What is {} times {}", puzzle.s1, puzzle.s2);

        let x = util::read();

        match x {
            Ok(x) => {
                let m = x.parse::<String>().unwrap().parse::<i32>();
                match m {
                    Ok(m) => {
                        println!("You entered the number {}", m);
                        if m == puzzle.n3 {
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
                println!("user error, try again, error was");
            }
        };
    }
}