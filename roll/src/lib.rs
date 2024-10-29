// 2d6 + 3d10
// 3d6 + 5
// {roll} {operator} {roll} ....

#[derive(Debug, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
}

#[derive(Debug, PartialEq)]
pub enum RollElement {
    Roll { count: usize, value: usize },
    Operator(Operator)
}

pub struct Roll {
    pub elements: Vec<RollElement>
}

impl Roll {
    pub fn parse(expr: String) -> Option<Self> {
        let mut expr = expr.replace(&[' ', '\t', '\n'][..], "");
        let mut elements: Vec<RollElement> = Vec::new();
        loop {
            println!("{}", expr);
            if let Some(oper) = expr.find(&['+', '-', '*'][..]) {
                let die_str = expr.split_at(oper).0.to_owned();
                let oper_chr = expr.chars().nth(oper).unwrap();
                println!("{:?} {:?}", die_str, oper_chr);
                expr = expr.chars().skip(oper+1).collect();
                elements.push(Roll::parse_die(&die_str.to_owned().to_string())?);
                elements.push(match oper_chr {
                    '+' => RollElement::Operator(Operator::Add),
                    '-' => RollElement::Operator(Operator::Sub),
                    '*' => RollElement::Operator(Operator::Mul),
                    _ => {unreachable!()}
                })
            } else {
                elements.push(Roll::parse_die(&expr)?);
                break;
            }
        }
        Some(Roll {elements})
    }
    pub fn parse_die(die_expr: &String) -> Option<RollElement> {
        let (count_s, value_s) = die_expr.split_once('d')?;
        let count = str::parse::<usize>(count_s).ok()?;
        let value = str::parse::<usize>(value_s).ok()?;
        Some(RollElement::Roll {count, value})
    }
}




#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse() -> Result<(), ()> {
        use RollElement as RE;
        let roll = Roll::parse("1d6 + 3d10".to_string()).ok_or(())?;
        assert_eq!(roll.elements, vec![RE::Roll{count: 1, value: 6}, RE::Operator(Operator::Add), RE::Roll{count: 3, value: 10}]);
        Ok(())
    }
}
