use std::time::Instant;

fn is_safe(nums: &[i32]) -> bool {
    if nums.len() < 2 {
        return false;
    }

    nums.windows(2).all(|pair| {
        let diff = (pair[1]-pair[0]).abs();
        diff <=3 && diff >0
    }) && (
        nums.windows(2).all(|pair| pair[0]<pair[1]) || 
        nums.windows(2).all(|pair| pair[0]>pair[1])
    )
}

fn first_challenge() {
    let contents = std::fs::read_to_string("input.txt").expect("fail to read from input.txt");

    let ans = contents
        .lines()
        .filter(|line| {
            is_safe(
                &line
                    .split_whitespace()
                    .map(|num| num.parse().unwrap())
                    .collect::<Vec<_>>()
            )
        })
        .count();

    println!("answer of the first challenge is: {}", ans);
}

fn can_be_made_safe(nums: &[i32]) -> bool {
    if nums.len() < 2 {
        return false;
    }

    (0..nums.len()).any(|i| {
        let mut nums_one = nums.to_vec();
        nums_one.remove(i);
        is_safe(&nums_one)
    })
}

fn second_challenge() {
    let contents = std::fs::read_to_string("input.txt").expect("fail to read from input.txt");

    let ans = contents
        .lines()
        .filter(|line| {
            let nums: Vec<i32> = line
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();

            is_safe(&nums) || can_be_made_safe(&nums)
        })
        .count();

    println!("answer of the second challenge is: {}", ans);
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
