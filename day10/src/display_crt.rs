pub enum DisplayCrt {
    Lit,
    Unlit,
}

impl DisplayCrt {
    pub fn to_char(&self) -> char {
        match self {
            DisplayCrt::Lit => '#',
            DisplayCrt::Unlit => '.',
        }
    }
}
