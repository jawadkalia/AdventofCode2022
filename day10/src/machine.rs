use crate::display_crt::DisplayCrt;

#[derive(Debug, Default)]
pub struct Machine {
    xreg: i32,
    cycle: i32,
    scanline: String,
    crt: Vec<String>,
}

impl Machine {
    pub fn init(&mut self) {
        self.xreg = 1;
        self.cycle = 0;
        self.scanline = "".to_string();
        self.crt.clear();
    }

    pub fn noop(&mut self) {
        self.cycle();
    }

    pub fn addx(&mut self, delta: i32) {
        self.cycle();
        self.cycle();
        self.xreg += delta;
    }

    pub fn cycle(&mut self) {
        if self.cycle % 40 == 0 && !self.scanline.is_empty() {
            self.crt.push(self.scanline.clone());
            self.scanline.clear();
        }

        match self.cycle % 40 < self.xreg - 1 && self.cycle % 40 > self.xreg + 1 {
            true => self.scanline.push(DisplayCrt::Unlit.to_char()),
            false => self.scanline.push(DisplayCrt::Lit.to_char()),
        }
        self.cycle += 1;
    }
}
