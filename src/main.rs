use std::env;
use std::fs;

struct Tape {
    tape: Vec<u32>,
    pointer: usize
}

impl Tape {
    fn new() -> Tape {
        Tape {
            tape: vec![0; 30000],
            pointer: 0
        }
    }
    
    fn inc_ptr(&mut self) {
        self.pointer += 1;
    }

    fn dec_ptr(&mut self) {
        self.pointer -= 1;
    }

    fn inc_val(&mut self) {
        self.tape[self.pointer] += 1;
    }

    fn dec_val(&mut self) {
        self.tape[self.pointer] -= 1;
    }

    fn set(&mut self, val: u32) {
        self.tape[self.pointer] = val;
    }

    fn get(&self) -> u32 {
        return self.tape[self.pointer];
    }
}

#[derive(Clone)]
enum Token {
    IncPtr,
    DecPtr,
    IncVal,
    DecVal,
    LoopStart,
    LoopEnd,
    Set,
    Get,
    Ignore
}

enum Instruction {
    IncPtr,
    DecPtr,
    IncVal,
    DecVal,
    Set,
    Get,
    Loop(Vec<Instruction>)
}

fn tokenize(contents: &String) -> Vec<Token> {
    let mut tokens = Vec::new();
    for ch in contents.chars() {
        let token: Token = match ch {
            '>' => Token::IncPtr,
            '<' => Token::DecPtr,
            '+' => Token::IncVal,
            '-' => Token::DecVal,
            '[' => Token::LoopStart,
            ']' => Token::LoopEnd,
            ',' => Token::Set,
            '.' => Token::Get,
            _ => Token::Ignore
        };
        tokens.push(token);
    }
    tokens
}

fn parse(tokens: &Vec<Token>) -> Vec<Instruction> {
    let mut program = Vec::new();
    let mut loop_stack = 0;
    let mut loop_start = 0;
    for (i, token) in tokens.iter().enumerate() {
        if loop_stack == 0 {
            let instruction: Instruction = match token {
                Token::IncPtr => Instruction::IncPtr,
                Token::DecPtr => Instruction::DecPtr,
                Token::IncVal => Instruction::IncVal,
                Token::DecVal => Instruction::DecVal,
                Token::Set => Instruction::Set,
                Token::Get => Instruction::Get,
                Token::LoopStart => {
                    loop_stack += 1;
                    loop_start = i;
                    continue;
                },
                Token::LoopEnd => panic!("Unmatched loop end"),
                Token::Ignore => continue
            };
            program.push(instruction);
        } else {
            match token {
                Token::LoopStart => loop_stack += 1,
                Token::LoopEnd => {
                    loop_stack -= 1;
                    if loop_stack == 0 {
                        let loop_tokens = tokens[loop_start+1..i].to_vec();
                        let loop_instructions = parse(&loop_tokens);
                        program.push(Instruction::Loop(loop_instructions));
                    }
                },
                _ => ()
            }
        }
    }
    program
}

fn run(program: &Vec<Instruction>, tape: &mut Tape) {
    for instruction in program {
        match instruction {
            Instruction::IncPtr => tape.inc_ptr(),
            Instruction::DecPtr => tape.dec_ptr(),
            Instruction::IncVal => tape.inc_val(),
            Instruction::DecVal => tape.dec_val(),
            Instruction::Set => tape.set(0),
            Instruction::Get => {
                let output = tape.get();
                let converted = std::char::from_u32(output).unwrap();
                print!("{}", converted);
            },
            Instruction::Loop(loop_instructions) => {
                while tape.get() != 0 {
                    run(loop_instructions, tape);
                }
            }
        }
    }
}

fn main() {
    // IO Stuff
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: bf-rust <file_path>");
        return;
    }
    let file_path: &String = &args[1];

    let contents: String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    // Tokenize
    let tokens: Vec<Token> = tokenize(&contents);
    
    // Parse
    let program: Vec<Instruction> = parse(&tokens);

    // Evaluate
    let mut tape = Tape::new();
    run(&program, &mut tape);
}
