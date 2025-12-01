use regex::Regex;

fn main() {
    let mut safe = Safe::new();
    safe.unlock();
    println!(
        "The actual password to open the door is {}",
        safe.zero_dials
    );
    println!(
        "Using password method 0x434C49434B, the password to open the door is {}",
        safe.zero_dials + safe.zero_passes
    );
}

struct Safe {
    /// Current value the dial is pointing at
    dial: i32,
    /// Number of times the dial pointed at zero
    zero_dials: u32,
    /// Number of times the dial passed but didn't stop by zero
    zero_passes: u32,
}

enum Direction {
    Left,
    Right,
}

struct Instruction {
    direction: Direction,
    value: i32,
}

impl Safe {
    pub fn new() -> Safe {
        Safe {
            dial: 50,
            zero_dials: 0,
            zero_passes: 0,
        }
    }

    pub fn unlock(&mut self) -> u32 {
        let instructions = Self::read_instructions();
        instructions.iter().for_each(|inst| self.execute(inst));
        self.zero_dials
    }

    pub fn read_instructions() -> Vec<Instruction> {
        let input = include_str!("../../../inputs/2025/d01/input.txt");
        let mut instructions = vec![];
        let re = Regex::new(r"([LR])(\d+)").unwrap();

        input.lines().for_each(|l| {
            for (_, [dir, val]) in re.captures_iter(l).map(|c| c.extract()) {
                instructions.push(Instruction {
                    direction: if dir == "L" {
                        Direction::Left
                    } else {
                        Direction::Right
                    },
                    value: val.parse().unwrap(),
                })
            }
        });

        instructions
    }

    pub fn execute(&mut self, instruction: &Instruction) {
        match instruction.direction {
            Direction::Left => {
                self.dial -= instruction.value;
                while self.dial < 0 {
                    self.dial += 100;
                    if self.dial != 0 {
                        self.zero_passes += 1
                    }
                }
            }
            Direction::Right => {
                self.dial += instruction.value;
                while self.dial >= 100 {
                    self.dial -= 100;
                    if self.dial != 0 {
                        self.zero_passes += 1
                    }
                }
            }
        }

        if self.dial == 0 {
            self.zero_dials += 1
        }
    }
}
