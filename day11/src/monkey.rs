use crate::operation::Operation;

#[derive(Debug, Default, Clone)]
pub struct Monkey {
    pub items: Vec<u32>,
    pub operation: Operation,
    pub worry_test: i32,
    pub counts: u32,
    pub destination: (u32, u32),
}

impl Monkey {
    pub fn from_words(words: &Vec<&str>) -> Self {
        let mut monkey = Monkey::default();
        match *words.first().unwrap() {
            "Monkey" => monkey = Monkey::default(),
            "Starting" => words[2..].iter().for_each(|f| {
                monkey
                    .items
                    .push(f.replace(',', "").parse::<u32>().unwrap());
            }),
            "Operation:" => match words[4] {
                "+" => match words[5] == "old" {
                    true => monkey.operation = Operation::AddSelf,
                    false => monkey.operation = Operation::Add(words[5].parse().unwrap()),
                },
                "*" => match words[5] == "old" {
                    true => monkey.operation = Operation::MultiplySelf,
                    false => monkey.operation = Operation::Multiply(words[5].parse().unwrap()),
                },
                _ => unreachable!(),
            },
            "Test:" => {
                monkey.worry_test = words[3].parse().unwrap();
            }
            "If" => match words[1] {
                "true:" => monkey.destination.0 = words[5].parse().unwrap(),
                "false:" => monkey.destination.1 = words[5].parse().unwrap(),
                _ => unreachable!(),
            },

            _ => monkey = Monkey::default(),
        }
        // dbg!(monkey.clone());
        monkey
    }
}
