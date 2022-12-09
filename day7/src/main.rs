use std::collections::HashMap;

fn main() {
    let mut dir_sizes: HashMap<String, u32> = HashMap::new();
    let mut current_path: Vec<String> = Vec::new();

    let value = include_str!("../input.txt").lines().collect::<Vec<_>>();
    let value2 = value
        .iter()
        .map(|f| f.split(" ").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();
    let value3 = value2
        .iter()
        .map(|f| match f.first().unwrap().to_owned() {
            "$" => {
                println!("COMMAND");
                dbg!(f.last().unwrap());
                Commands::from_str(f.last().unwrap()).update_path(&mut current_path);
            }
            "dir" => {
                println!("dir");
                dbg!(f.clone().last().unwrap());
                current_path.push(f.last().unwrap().to_string());
            }
            _ => {
                println!("numerical")
            }
        })
        .collect::<Vec<_>>();

    
}

enum Commands {
    CD_TOP_LEVEL,
    CD_MOVE_UP,
    ls,
    Dir(String),
}

impl Commands {
    fn from_str(s: &str) -> Commands {
        match s {
            "/" => Commands::CD_TOP_LEVEL,
            "ls" => Commands::ls,
            ".." => Commands::CD_MOVE_UP,
            _ => Commands::Dir(s.to_string()),
        }
    }

    fn update_path(self, path: &mut Vec<String>) {
        match self {
            Commands::CD_TOP_LEVEL => {
                path.clear();
                path.push("/".to_string());
            }

            Commands::CD_MOVE_UP => {
                path.pop();
            }
            Commands::ls => {
                path.join("/");
            }
            Commands::Dir(a) => {
                path.push(a)
            },
        }
    }
}
