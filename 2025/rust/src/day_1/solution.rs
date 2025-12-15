use std::fs::File;
use std::io::Read;

pub fn read_input(path: &str) -> String {
    let mut data = String::new();
    let mut f = File::open(path).expect("Should be able to open file");
    f.read_to_string(&mut data)
        .expect("Should be able to read string");
    return data;
}

pub fn str_to_str_list(string: &String) -> Vec<&str> {
    let parts = string.split("\n");
    return parts.filter(|&x| x != "").collect::<Vec<&str>>();
}

pub fn process_steps(steps: Vec<&str>) -> (i32, i32) {
    let mut state: i32 = 50;
    let mut count: i32 = 0;
    for step in steps {
        let value = step[1..].parse::<i32>().unwrap();
        match &step[0..1] {
            "L" => state = state - value,
            "R" => state = state + value,
            x => panic!("unsupported op: {x}"),
        }
        if state < 0 || state > 99 {
            //Workaround for modulo using remainder
            state = ((state % 100)+100)%100;
        }

        if state == 0 {
            count += 1;
        }
        println!("Step: {step}, State: {state}, Count: {count}");
    }
    return (state, count);
}
