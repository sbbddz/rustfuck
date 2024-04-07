struct BrainfuckMachine {
    code: &'static str,
    sp: usize,
    memory: [u8; 30000],
    dp: usize,
}

impl BrainfuckMachine {
    pub fn new(code: &'static str) -> BrainfuckMachine {
        BrainfuckMachine {
            code: code.trim(),
            sp: 0,
            memory: [0; 30000],
            dp: 0,
        }
    }

    fn run(&mut self) -> () {
        while self.sp < self.code.len() {
            match self.get_current_char() {
                "+" => self.memory[self.dp] += 1,
                "-" => self.memory[self.dp] -= 1,
                ">" => self.dp += 1,
                "<" => self.dp -= 1,
                "." => print!("{}", self.memory[self.dp] as char),
                "," => todo!("Implement reading from input"),
                "[" => self.handle_branch_start(),
                "]" => self.handle_branch_end(),
                c => panic!(
                    "Invalid character found when parsing the Brainfuck file: {}",
                    c
                ),
            }
            self.sp += 1;
        }
    }

    /// Execute the following code only if the MEMORY[DP] != 0.
    /// Handle multiple depths
    fn handle_branch_start(&mut self) -> () {
        if self.memory[self.dp] == 0 {
            let mut depth = 1;
            while depth != 0 {
                self.sp += 1;
                match self.get_current_char() {
                    "[" => depth += 1,
                    "]" => depth -= 1,
                    _ => continue,
                }
            }
        }
    }

    fn handle_branch_end(&mut self) -> () {
        if self.memory[self.dp] != 0 {
            let mut depth = 1;
            while depth != 0 {
                self.sp -= 1;
                match self.get_current_char() {
                    "[" => depth -= 1,
                    "]" => depth += 1,
                    _ => continue,
                }
            }
        }
    }

    fn get_current_char(&self) -> &str {
        &self.code[self.sp..=self.sp]
    }
}

fn main() {
    let mut machine = BrainfuckMachine::new(include_str!("file.bf"));
    machine.run();
}
