use std::time::Instant;

fn first_challenge() {
    let contents = std::fs::read_to_string("input.txt").expect("fail to read from input.txt");
    let mut ans = 0;
    let mut index = 0;
    while let Some(start_pos) = contents[index..].find("mul(") {
        let abs_pos = index + start_pos;
        
        if let Some(end_pos) = contents[abs_pos..].find(')') {
            let full_end = abs_pos + end_pos;
            let candidate = &contents[abs_pos..=full_end];
            
            if is_valid_mul_pattern(candidate) {
                let params = &candidate[4..candidate.len()-1]; 
                if let Some((a, b)) = params.split_once(',') {
                    ans += a.trim().parse::<i32>().unwrap() * b.trim().parse::<i32>().unwrap();
                }
            }
            
            index = abs_pos + 4; 
        } else {
            break;
        }
    }
    println!("{}", ans);}

fn is_valid_mul_pattern(s: &str) -> bool {
    if !s.starts_with("mul(") || !s.ends_with(')') || s.as_bytes()[3] != b'(' {
        return false;
    }
    
    let inner = &s[4..s.len()-1];
    
    if inner.matches(',').count() != 1 {
        return false;
    }
    
    if let Some((a, b)) = inner.split_once(',') {
        let a = a.trim();
        let b = b.trim();
        
        a.chars().all(|c| c.is_ascii_digit()) && 
        b.chars().all(|c| c.is_ascii_digit())
    } else {
        false
    }
}

fn second_challenge() {
    let contents = std::fs::read_to_string("input.txt").expect("fail to read from input.txt");
    let mut ans = 0;
    let mut enabled = true; 
    let mut index = 0;
    
    while index < contents.len() {
        let remaining = &contents[index..];
        
        if remaining.starts_with("mul(") && enabled {
            if let Some(end_pos) = remaining.find(')') {
                let candidate = &remaining[..=end_pos];
                
                if is_valid_mul_pattern(candidate) {
                    let params = &candidate[4..candidate.len()-1];
                    if let Some((a, b)) = params.split_once(',') {
                        ans += a.trim().parse::<i32>().unwrap() * b.trim().parse::<i32>().unwrap();
                    }
                }
                index += 1;
                continue;
            }
        } else if remaining.starts_with("do()") {
            enabled = true;
            index += 4;
            continue;
        } else if remaining.starts_with("don't()") {
            enabled = false;
            index += 7;
            continue;
        }
        
        index += 1;
    }
    
    println!("Sum of enabled multiplications: {}", ans);
}

fn main() {
    let start = Instant::now();
    
    first_challenge();
    let first_duration = start.elapsed();
    
    let second_start = Instant::now();
    second_challenge();
    let second_duration = second_start.elapsed();
    
    let total_duration = start.elapsed();
    
    println!("First challenge took: {:?}", first_duration);
    println!("Second challenge took: {:?}", second_duration);
    println!("Total execution time: {:?}", total_duration);
}
