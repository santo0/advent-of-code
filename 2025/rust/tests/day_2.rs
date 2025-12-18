use rust::common::utils::str_to_str_list;
use rust::day_2::solution::solve_a;
use rust::day_2::solution::solve_b;


const TEST_VAL: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

#[test]
fn test_first_star(){
    let list = str_to_str_list(TEST_VAL, ",");
    let sol = solve_a(list);
    assert_eq!(sol, 1227775554);
}

#[test]
fn test_second_star(){
    let list = str_to_str_list(TEST_VAL, ",");
    let sol = solve_b(list);
    assert_eq!(sol, 4174379265);
}
