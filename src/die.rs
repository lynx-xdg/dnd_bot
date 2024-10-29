use rand;
use rand::Rng;

#[derive(Debug)]
pub struct Roll {
    //     ex:   4   d  6
    pub dice: Vec<(usize, usize)>
}

impl Roll {
    pub fn parse(string: &str) -> Option<Roll> {
        let throws = string.split("+");
        let mut dice = Vec::new();
        for throw in throws {
            let throw = throw.trim();
            let mut split = throw.split("d");
            if let Some(count_str) = split.next() {
                if let Ok(count) = count_str.parse::<usize>() {
                    if let Some(dtype_str) = split.next() {
                        if let Ok(dtype) = dtype_str.parse::<usize>() {
                            // valid dtype
                            dice.push((count, dtype));
                        } else {
                            return None
                        }
                    } else {
                        // dtype not found -> d1
                        dice.push((count, 1));
                    }
                } else {
                    return None
                }
            }
        }
        Some(Roll {dice})
    }
    pub fn eval(&self) -> usize {
        let mut roll: usize = 0;
        let mut rng = rand::thread_rng();
        for dice in &self.dice {
            for _ in 0..dice.0 {
                roll += rng.gen_range(0..dice.1);
            }
        }
        roll
    }
    pub fn eval_vec(&self) -> Vec<(usize, usize)> {
        let mut roll: Vec<(usize, usize)> = Vec::new();
        let mut rng = rand::thread_rng();
        for dice in &self.dice {
            for _ in 0..dice.0 {
                roll.push((rng.gen_range(1..(dice.1 + 1)), dice.1));
            }
        }
        roll
    }
}

impl std::fmt::Display for Roll {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.dice))?;
        Ok(())
    }
}
