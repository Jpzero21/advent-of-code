use regex::Regex;

fn main() {
    let mut safe = Safe::new();
    let password = safe.unlock();
    println!("The actual password to open the door is {}", password);
}

struct Safe {
    /// Current value the dial is pointing at
    dial: u32,
    /// Number of times the dial pointed at zero
    zero_dials: u32,
}

enum Direction {
    Left,
    Right,
}

struct Instruction {
    direction: Direction,
    value: u32,
}

impl Safe {
    pub fn new() -> Safe {
        Safe {
            dial: 50,
            zero_dials: 0,
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
        todo!()

        match instruction.direction {
            Direction::Left => {
                if instruction.value > self.dial {
                    self.dial = 100 - (instruction.value - self.dial);
                } else {
                    self.dial -= instruction.value;
                }
            }
            Direction::Right => {
                if (self.dial + instruction.value) > 99 {
                    self.dial = self.dial + instruction.value - 100;
                } else {
                    self.dial += instruction.value;
                }
            }
        }

        if self.dial == 0 {
            self.zero_dials += 1
        }
    }
}
