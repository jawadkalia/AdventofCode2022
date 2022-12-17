use crate::day11::Day11;

pub fn round(day: &mut Day11) {
    // dbg!(day.monkeys.len());
    for i in 0..day.monkeys.len() {
        while let Some(item) = day.monkeys.get_mut(&i).unwrap().items.pop_front() {
            day.monkeys.get_mut(&i).unwrap().counts += 1;
            // dbg!(item);
            let worry = day.monkeys.get(&i).unwrap().operation.calc(item) / 3;
            // dbg!(worry);
            let dest = match worry % day.monkeys.get(&i).unwrap().worry_test == 0 {
                true => day.monkeys.get(&i).unwrap().destination.0,
                false => day.monkeys.get(&i).unwrap().destination.1,
            };

            // dbg!(dest);

            day.monkeys
                .get_mut(&dest.try_into().unwrap())
                .unwrap()
                .items
                .push_back(worry);
        }
    }
}
