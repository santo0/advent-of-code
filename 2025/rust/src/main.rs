mod day_1;
use day_1::solution::read_input;
use day_1::solution::str_to_str_list;
use day_1::solution::process_steps;

fn main() {
    println!("Hello, world!");
    let input: String = read_input("/home/user/GitHub/advent-of-code/2025/rust/.input/day_1.txt");
    //println!("{input}");
    let list = str_to_str_list(&input);
    //println!("{list:?}");
    let sol = process_steps(list);
    println!("{sol:?}");
}
