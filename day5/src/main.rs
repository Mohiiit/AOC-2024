use std::time::Instant;
use std::collections::HashMap;
use std::collections::HashSet;

fn is_valid_series(hash_map: &HashMap<u64, HashSet<u64>>, series: &[u64]) -> bool {
    for i in 0..series.len() {
        for j in 0..i {
            if hash_map.contains_key(&series[i]) && hash_map[&series[i]].contains(&series[j]) {
                return false;
            }
        }
    }
    true
}

fn first_challenge() {
    let contents = std::fs::read_to_string("input.txt")
        .expect("fail to read from input.txt");
    let mut rule_over = false;
    let mut hash_map: HashMap<u64, HashSet<u64>> = HashMap::new();
    let mut ans = 0;
    for line in contents.lines() {
        if(line.is_empty()) {
            rule_over = true;
            continue;
        }
        if !rule_over {
            let (left, right) = line.split_once('|').unwrap();
            let left_num = left.trim().parse::<u64>().unwrap();
            let right_num = right.trim().parse::<u64>().unwrap();
            if hash_map.contains_key(&left_num) {
                hash_map.get_mut(&left_num).unwrap().insert(right_num);
            } else {
                let mut set: HashSet<u64> = HashSet::new();
                set.insert(right_num);
                hash_map.insert(left_num, set);
            }
        } else {
            let series = line.split(',').map(|num| num.trim().parse::<u64>().unwrap()).collect::<Vec<_>>();
            if is_valid_series(&hash_map, &series) {
                ans += series[series.len()/2];
            }
        }
    }
    println!("answer of the first challenge is {}", ans);
}



fn second_challenge() {
    let contents = std::fs::read_to_string("sample_input.txt")
        .expect("fail to read from input.txt");
   
    
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
