fn is_invalid_a(num: i64) -> bool {
    let num_str = num.to_string();
    let len = num_str.len();
    if len % 2 != 0 {
        // Odd length number can't be invalid
        return false;
    }
    return num_str[0..len / 2] == num_str[len / 2..len];
}

pub fn solve_a(ranges: Vec<&str>) -> i64 {
    let mut total: i64 = 0;
    for range in ranges {
        let parts: Vec<&str> = range.split('-').map(|s| s.trim()).collect();
        let start: &str = parts[0];
        let end: &str = parts[1];
        let start_num: i64 = start.parse().unwrap();
        let end_num: i64 = end.parse().unwrap();
        for num in start_num..=end_num {
            if is_invalid_a(num) {
                total += num;
            }
        }
    }
    return total;
}

fn is_invalid_b(num: i64) -> bool {
    let num_str = num.to_string();
    let len = num_str.len();
    if num == 100 {
        println!("Debug");
    }

    // Check if num is made of repeating substrings
    for sub_size in 1..=(len / 2) {
        // Only consider if length is multiple of num_rep
        if len % sub_size == 0 {
            let base = &num_str[0..sub_size];
            let mut all_same = true;
            for i in (sub_size..len).step_by(sub_size) {
                if &num_str[i..i + sub_size] != base {
                    all_same = false;
                }
            }
            if all_same {
                return true;
            }
        }
    }
    return false;
}

pub fn solve_b(ranges: Vec<&str>) -> i64 {
    let mut total: i64 = 0;
    for range in ranges {
        let parts: Vec<&str> = range.split('-').map(|s| s.trim()).collect();
        let start: &str = parts[0];
        let end: &str = parts[1];
        let start_num: i64 = start.parse().unwrap();
        let end_num: i64 = end.parse().unwrap();
        for num in start_num..=end_num {
            if is_invalid_b(num) {
                total += num;
            }
        }
    }
    return total;
}
