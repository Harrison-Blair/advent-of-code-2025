use std::fs;

fn read_file(file_name: &str) -> (Vec<Vec<u128>>, Vec<char>) {
    let mut rows: Vec<Vec<u128>> = Vec::new();
    let mut ops: Vec<char> = Vec::new();

    let text: String = fs::read_to_string(file_name).expect("[N] Failed to read from file!");

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
            rows.push(row);
        }
    }

    // Transpose rows to get problems (columns)
    // Assuming all rows have the same length
    let num_problems = if !rows.is_empty() { rows[0].len() } else { 0 };
    let mut problems: Vec<Vec<u128>> = vec![Vec::new(); num_problems];

    for row in rows {
        for (i, val) in row.into_iter().enumerate() {
            if i < num_problems {
                problems[i].push(val);
            }
        }
    }

    (problems, ops)
}

fn read_file_right_left(file_name: &str) -> Vec<Vec<u128>> {
    let text: String = fs::read_to_string(file_name).expect("[RL] Failed to read from file!");
    let lines: Vec<&str> = text.lines().collect();
    
    // Exclude the last line (ops)
    let data_lines = if !lines.is_empty() { &lines[..lines.len() - 1] } else { &[] };
    
    if data_lines.is_empty() {
        return Vec::new();
    }

    let max_len = data_lines.iter().map(|l| l.len()).max().unwrap_or(0);
    
    // Transpose characters
    let mut cols: Vec<Vec<char>> = vec![Vec::new(); max_len];
    for line in data_lines {
        for (i, c) in line.chars().enumerate() {
            cols[i].push(c);
        }
        // Pad with spaces if line is short
        for i in line.len()..max_len {
            cols[i].push(' ');
        }
    }

    let mut problems: Vec<Vec<u128>> = Vec::new();
    let mut current_problem: Vec<u128> = Vec::new();
    let mut in_problem = false;

    for col in cols {
        // Check if column is empty (all spaces)
        let is_empty = col.iter().all(|c| *c == ' ');

        if is_empty {
            if in_problem {
                problems.push(current_problem);
                current_problem = Vec::new();
                in_problem = false;
            }
        } else {
            in_problem = true;
            // Parse number from column
            let s: String = col.into_iter().filter(|c| *c != ' ').collect();
            if let Ok(n) = s.parse::<u128>() {
                current_problem.push(n);
            }
        }
    }
    
    if in_problem {
        problems.push(current_problem);
    }

    problems
}

fn evaluate_nums(problems: Vec<Vec<u128>>, ops: Vec<char>) -> u128 {
    let mut total: u128 = 0;

    for (i, operands) in problems.iter().enumerate() {
        if i >= ops.len() { break; }
        let op = ops[i];
        
        if operands.is_empty() { continue; }

        let mut result = operands[0];
        for val in &operands[1..] {
            if op == '*' {
                result *= val;
            } else {
                result += val;
            }
        }
        total += result;
    }

    total
}

fn main() {
    let file_name = "day-6/input.txt";
    println!("Adv-of-Code [DAY-6] | Input File : {file_name}");

    let (problems, ops) = read_file(file_name);

    let p1_solution = evaluate_nums(problems, ops.clone());

    let c_problems = read_file_right_left(file_name);

    let p2_solution = evaluate_nums(c_problems, ops);

    println!("PART-1 [RESULT] | {p1_solution}");
    println!("PART-2 [RESULT] | {p2_solution}");
}
