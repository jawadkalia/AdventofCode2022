fn main() {
    let mut value = include_str!("../input.txt").to_string().chars().collect::<Vec<char>>();
    if *value.last().unwrap() == '\n' {
        value.pop();
    }
    let first_marker = marker_finder(value.clone(), 4);
    let second_marker = marker_finder(value, 14);
    
    dbg!(first_marker);
    dbg!(second_marker);

}

fn marker_finder(value: Vec<char>, width: usize) -> usize {
    'next_window: for (index, window) in value.windows(width).enumerate(){
        for i in 0..width - 1 {
            for j in i + 1..width {
                if window[i] == window[j] {
                    continue 'next_window
                }
            }
        }
        return index + width;
    }
    0
}
