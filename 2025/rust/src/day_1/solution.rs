fn modulo(val: i32, modulus: i32) -> i32 {
    ((val % modulus) + modulus) % modulus
}

pub fn process_steps_a(steps: Vec<&str>) -> (i32, i32) {
    let mut state: i32 = 50;
    let mut count: i32 = 0;
    for step in steps {
        let value = step[1..].parse::<i32>().unwrap();

        match &step[0..1] {
            "L" => state = modulo(state - value, 100),
            "R" => state = modulo(state + value, 100),
            x => panic!("unsupported op: {x}"),
        }

        count += if state == 0 { 1 } else { 0 };

        println!("Step: {step}, State: {state}, Count: {count}");
    }
    return (state, count);
}

pub fn process_steps_b(steps: Vec<&str>) -> (i32, i32) {
    let mut state: i32 = 50;
    let mut count: i32 = 0;
    for step in steps {
        let value = step[1..].parse::<i32>().unwrap();
        let mut laps = match &step[0..1] {
            "L" => ((state - value) / 100).abs(),
            "R" => (state + value) / 100,
            x => panic!("unsupported op: {x}"),
        };

        laps = if &step[0..1] == "L" && state != 0 && state - value <= 0 {
            laps + 1
        } else {
            laps
        };

        match &step[0..1] {
            "L" => state = modulo(state - value, 100),
            "R" => state = modulo(state + value, 100),
            x => panic!("unsupported op: {x}"),
        }

        count += laps;
        println!("Step: {step}, State: {state}, Count: {count}");
    }
    return (state, count);
}
