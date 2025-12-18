use rust::common::utils::str_to_str_list;
use rust::day_1::solution::process_steps_a;
use rust::day_1::solution::process_steps_b;

const TEST_VAL: &str = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#;
        
#[test]
fn test_first_star(){
    let list = str_to_str_list(TEST_VAL, "\n");
    let sol = process_steps_a(list);
    assert_eq!(sol, (32, 3));
}

#[test]
fn test_second_star(){
    let list = str_to_str_list(TEST_VAL, "\n");
    let sol = process_steps_b(list);
    assert_eq!(sol, (32, 6));
}
