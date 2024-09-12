use crate::util::TokenLoc;

pub struct ErrHandler {
    src: Vec<String>,
}

pub type Result<T> = std::result::Result<T, (TokenLoc, String)>;

impl ErrHandler {
    pub fn new(src: &str) -> Self {
        Self {
            src: src.split('\n').into_iter().map(|s| s.into()).collect(),
        }
    }

    pub fn handle<T>(&self, res: Result<T>) -> T {
        res.unwrap_or_else(|(loc, s)| {
            eprintln!("{}", s);
            if !loc.is_eof() {
                if loc.line != 1 {
                    eprintln!("    {}", self.src[loc.line - 2]);
                }
                eprintln!("    {}", self.src[loc.line - 1]);

                eprint!("\x1B[31m");
                eprint!("    ");
                self.src[loc.line - 1]
                    .chars()
                    .enumerate()
                    .take_while(|(i, c)| *i < (loc.col - 1))
                    .for_each(|(_, c)| {
                        if c == '\t' {
                            eprint!("\t");
                        } else {
                            eprint!(" ");
                        }
                    });
                eprint!("^");
                for _ in 0..(loc.len - 1) {
                    eprint!("~");
                }
                eprintln!("\x1B[0m");

                if loc.line < self.src.len() {
                    eprintln!("    {}", self.src[loc.line]);
                }
            }
            std::process::exit(1);
        })
    }
}
