use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args[(args.len()) - 1] == args[0] {
        panic!("Please insert a file!")
    }

    // file and parsed file are designed to
    let file = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");
    let parsedfile: Vec<char> = file.chars().collect();
    // input is required for the comma action
    let mut input = String::new();
    // Memory, pointer and last loop is used for the interpretation

    // According to a article I read, there are 30'0000 memory blocks.
    let mut memory: [u8; 300000] = [0; 300000];
    let mut pointer = 0;
    let mut lastloop = 0;
    let mut i = 0;

    while i != file.len() {
        /*
            My current method for interpriting is to use a match system.
            It is pretty unoptimized as of right now although I plan to change it for a
            more modern and accepted method to make an interpr

        */
        match parsedfile[i] {
            '+' => {
                if memory[pointer] == 255 {
                    memory[pointer] = 0;
                } else {
                    memory[pointer] += 1;
                }
            }
            '-' => {
                if memory[pointer] == 0 {
                    memory[pointer] = 255;
                } else {
                    memory[pointer] -= 1;
                }
            }
            '>' => {
                if pointer == (memory.len()) - 1 {
                    pointer = 0;
                } else {
                    pointer += 1;
                }
            }
            '<' => {
                if pointer == 0 {
                    pointer = (memory.len()) - 1;
                } else {
                    pointer -= 1;
                }
            }
            '.' => {
                print!("{}", memory[pointer] as char);
            }
            //I am stupid so I need to fix my dum dum goto and use a push pop method
            '[' => lastloop = i,
            ']' => {
                if memory[pointer] != 0 {
                    i = lastloop;
                }
            }
            ',' => {
                let _string = std::io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");
                #[warn(clippy::bytes_nth)]
                let bytes = input.as_bytes();
                memory[pointer] = bytes[0];
            }
            _ => (),
        };
        i += 1;
    }
}
