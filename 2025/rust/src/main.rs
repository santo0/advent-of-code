use rust::common::utils::read_input;
use rust::common::utils::str_to_str_list;
use rust::day_1::solution::process_steps_b;

fn main() {
    println!("Hello, world!");
    let input: String = read_input(".input/day_1.txt");
    //println!("{input}");
    let list = str_to_str_list(&input);
    //println!("{list:?}");
    let sol = process_steps_b(list);
    println!("{sol:?}");
}
