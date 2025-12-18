use rust::common::utils::read_input;
use rust::common::utils::str_to_str_list;
use rust::day_2::solution::solve_b;

fn main() {
    let input: String = read_input(".input/day_2.txt");
    let list = str_to_str_list(&input, ",");
    let sol = solve_b(list);
    println!("{sol:?}");
}
