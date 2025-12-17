use std::fs;

fn read_file(file_name: &str) -> (Vec<Vec<u128>>, Vec<char>) {
    let mut nums: Vec<Vec<u128>> = Vec::new();
    let mut ops: Vec<char> = Vec::new();

    let text: String = fs::read_to_string(file_name)
        .expect("[N] Failed to read from file!");

    for line in text.lines() {
        let mut row: Vec<u128> = Vec::new();
        for item in line.split_whitespace() {
            if let Ok(num) = item.parse::<u128>() {
                row.push(num);
            } else {
                ops.push(item.chars().next().unwrap());
            }
        }
        // Only push non-empty rows (skip the operator line)
        if !row.is_empty() {
            nums.push(row);
        }
    }

    (nums, ops)
}

fn read_file_right_left(file_name: &str) -> Vec<Vec<u128>> {
    let mut nums: Vec<Vec<u128>> = Vec::new();

    let text: String = fs::read_to_string(file_name)
        .expect("[RL] Failed to read from file!");

    

    nums
}

fn evaluate_nums(nums: Vec<Vec<u128>>, ops: Vec<char>) -> u128 {
    let mut total: u128 = 0;

    for i in 0..ops.len() {
        let mut result = nums[0][i];
        for row_idx in 1..nums.len() {
            if ops[i] == '*' {
                result *= nums[row_idx][i];
            } else {
                result += nums[row_idx][i]
            }
        }
        total += result;
    }

    total
}

fn main() {
    let file_name = "day-6/input.txt";
    println!("Adv-of-Code [DAY-6] | Input File : {file_name}");

    let (nums, ops) = read_file(file_name);

    let p1_solution = evaluate_nums(nums, ops.clone());

    let c_nums = read_file_right_left(file_name);

    let p2_solution: u128 = evaluate_nums(c_nums, ops.clone());

    println!("PART-1 [RESULT] | {p1_solution}");
    println!("PART-2 [RESULT] | {p2_solution}");
}
