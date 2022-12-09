use std::collections::HashMap;

fn main() {
    let mut dirs: HashMap<String, u32> = HashMap::new();

    let value = include_str!("../test.txt").lines().collect::<Vec<_>>();
    let value2 = value
        .iter()
        .map(|f| f.trim().split(" ").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();
    let mut current_path: Vec<String> = Vec::new();
    value2.iter().for_each(|f| {
        match f.first().unwrap().to_owned() {
            "$" => {
                Commands::from_str(f.last().unwrap()).update_path(&mut current_path);
                // dbg!(current_path.clone());
            }
            "dir" => {
                // current_path.pop();
                current_path.push(f.last().unwrap().to_string());
                // dbg!(current_path.clone());
            }
            _s => {
                // current_path.push(f.last().unwrap().to_string());
                // dbg!(current_path.clone());
            }
        }
        let pwd = current_path.join("/");
        if !dirs.contains_key(&pwd) {
            dirs.insert(pwd, 0);
        }
    });

    let mut dirs1: HashMap<String, u32> = HashMap::new();
    dirs1.clone_from(&dirs);

    let mut dirs2: HashMap<String, u32> = HashMap::new();
    dirs2.clone_from(&dirs);

    for (directory_path, size) in dirs1 {
        for d in dirs.keys() {
            if directory_path.starts_with(d) && !directory_path.eq(d) {
                let dir_size = dirs2.get(d).unwrap();

                dirs2.insert(d.to_string(), dir_size + size);
            }
        }
    }

    let answer1: u32 = dirs2.values().filter(|&f| *f < 100_000).sum();

    println!("Total size: {answer1}");
}

enum Commands {
    CdTopLevel,
    CdMoveUp,
    Ls,
    Dir(String),
}

impl Commands {
    fn from_str(s: &str) -> Commands {
        match s {
            "/" => Commands::CdTopLevel,
            "ls" => Commands::Ls,
            ".." => Commands::CdMoveUp,
            _ => Commands::Dir(s.to_string()),
        }
    }

    fn update_path(self, path: &mut Vec<String>) {
        match self {
            Commands::CdTopLevel => {
                path.clear();
                path.push("/".to_string());
            }
            Commands::CdMoveUp => {
                path.pop();
            }
            Commands::Ls => {
                path.push("/".to_string());
            }
            Commands::Dir(a) => {
                dbg!(path.clone());
                path.push(a);
            }
        }
    }
}
