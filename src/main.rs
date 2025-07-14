use anyhow::{Result, anyhow};
use std::{
    env::args,
    fs,
    io::{Read, Write, stdin, stdout},
};

fn main() -> Result<()> {
    let file = args()
        .skip(1)
        .next()
        .ok_or(anyhow!("Please specify a file"))?;

    let program = fs::read_to_string(file)?.chars().collect::<Vec<_>>();
    let mut memory = vec![0u8; 0xFFFF];

    let mut ptr = 0usize;
    let mut pc = 0usize;

    let mut stdout = stdout();
    let mut stdin = stdin();

    while pc < program.len() {
        match program[pc] {
            '>' => ptr = ptr.wrapping_add(1),
            '<' => ptr = ptr.wrapping_sub(1),
            '+' => memory[ptr] = memory[ptr].wrapping_add(1),
            '-' => memory[ptr] = memory[ptr].wrapping_sub(1),
            '.' => {
                stdout.write_all(&[memory[ptr]])?;
            }
            ',' => {
                let mut input = [0];
                stdin.read_exact(&mut input).unwrap_or(());
                memory[ptr] = input[0];
            }
            '[' => {
                if memory[ptr] == 0 {
                    let mut bracket_count = 1usize;

                    while pc < program.len() - 1 && bracket_count > 0 {
                        pc = pc.wrapping_add(1);

                        match program[pc] {
                            '[' => bracket_count += 1,
                            ']' => bracket_count -= 1,
                            _ => {}
                        }
                    }
                }
            }
            ']' => {
                if memory[ptr] != 0 {
                    let mut bracket_count = 1usize;

                    while pc > 0 && bracket_count > 0 {
                        pc = pc.wrapping_sub(1);

                        match program[pc] {
                            ']' => bracket_count += 1,
                            '[' => bracket_count -= 1,
                            _ => {}
                        }
                    }
                }
            }
            _ => {}
        }

        pc = pc.wrapping_add(1);
    }

    Ok(())
}
