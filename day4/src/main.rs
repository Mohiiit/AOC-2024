use std::time::Instant;

fn check_xmas(matrix: &Vec<Vec<char>>, x: i32, y: i32) -> u64 {
    let mut xmas: u64 = 0;
    let x_pos: [i32; 8] = [0,1,1,-1,0,-1,-1,1];
    let y_pos: [i32; 8] = [1,1,0,1,-1,-1,0,-1];
    let xmas_chars: [char; 3] = ['M', 'A', 'S'];
    for i in 0..8 {
        let x_new = x + x_pos[i] ;
        let y_new = y + y_pos[i];
        if x_new < 0 || y_new < 0 || x_new >= matrix.len() as i32 || y_new >= matrix[0].len() as i32 {
            continue;
        }
      
        let mut is_xmas = false;
        for j in 1..4 {
            let x_new = x + x_pos[i]*j;
            let y_new = y + y_pos[i]*j;
            if x_new < 0 || y_new < 0 || x_new >= matrix.len() as i32 || y_new >= matrix[0].len() as i32 {
                is_xmas = false;
                break;
            }
            if matrix[x_new as usize][y_new as usize] == xmas_chars[j as usize - 1] {
                is_xmas = true;
            } else {
                is_xmas = false;
                break;
            }
        }
        if is_xmas {
            xmas += 1;
        }
    }
    xmas
}

fn check_x_mas(matrix: &Vec<Vec<char>>, x: usize, y: usize) -> u64 {
    let mut xmas: u64 = 0;
    let x_pos: [i32; 4] = [1,-1,1,-1];
    let y_pos: [i32; 4] = [1,1,-1,-1];
    let mut is_valid = true;
    for i in 0..4 {
        let x_new = x as i32 + x_pos[i];
        let y_new = y as i32 + y_pos[i];
        if x_new < 0 || y_new < 0 || x_new >= matrix.len() as i32 || y_new >= matrix[0].len() as i32 || matrix[x_new as usize][y_new as usize] == 'X' || matrix[x_new as usize][y_new as usize] == 'A' {
            is_valid = false;
            break;
        }
    }
    if is_valid && matrix[x-1][y-1]!=matrix[x+1][y+1] && matrix[x-1][y+1]!=matrix[x+1][y-1] {
        xmas += 1;
    }
    xmas
}

fn first_challenge() {
    let contents = std::fs::read_to_string("input.txt").expect("fail to read from input.txt");
    let matrix = contents.lines().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut xmas_count: u64 = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == 'X' {
                xmas_count += check_xmas(&matrix, i as i32, j as i32);
            }
        }
    }
    println!("answer for the first challenge is {}", xmas_count);
}


fn second_challenge() {
    let contents = std::fs::read_to_string("input.txt").expect("fail to read from input.txt");
    let matrix = contents.lines().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut xmas_count: u64 = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == 'A' {
                xmas_count += check_x_mas(&matrix, i, j);
            }
        }
    }
    
    println!("answer for the second challenge is {}", xmas_count);
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
