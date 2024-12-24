use std::time::Instant;
//answer for the first challenge is 2378
//answer for the second challenge is 1796
fn is_in_bounds(x: i32, y: i32, matrix: &Vec<Vec<char>>) -> bool {
    x >= 0 && y >= 0 && x < matrix.len() as i32 && y < matrix[0].len() as i32
}

fn check_xmas(matrix: &Vec<Vec<char>>, x: i32, y: i32) -> u64 {

    const DIRECTIONS: [(i32, i32); 8] = [(0,1), (1,1), (1,0), (1,-1), (0,-1), (-1,-1), (-1,0), (-1,1)];
    const XMAS_CHARS: [char; 3] = ['M', 'A', 'S'];

    DIRECTIONS.iter().filter(|(dx,dy)| {
        (1..4).all(|step| {
            let x_new = x + dx * step;
            let y_new = y + dy * step;
            is_in_bounds(x_new, y_new, matrix) && matrix[x_new as usize][y_new as usize] == XMAS_CHARS[step as usize - 1]
        })
    }).count() as u64
   
}

fn check_x_mas(matrix: &Vec<Vec<char>>, x: usize, y: usize) -> u64 {
    const DIRECTIONS: [(i32, i32); 4] = [(1,-1), (1,1), (-1,-1), (-1,1)];

    let valid_positions = DIRECTIONS.iter().all(|(dx,dy)| {
            let x_new = x as i32 + dx;
            let y_new = y as i32 + dy;
            is_in_bounds(x_new, y_new, matrix) &&
            !matches!(matrix[x_new as usize][y_new as usize], 'X' | 'A')
    });

    let diagonals_pair_differ = valid_positions && {
        let top_left = matrix[x-1][y-1];
        let top_right = matrix[x+1][y+1];
        let bottom_left = matrix[x-1][y+1];
        let bottom_right = matrix[x+1][y-1];
        top_left != top_right && bottom_left != bottom_right
    };
    diagonals_pair_differ as u64
}

fn first_challenge() {
    let contents = std::fs::read_to_string("input.txt")
        .expect("fail to read from input.txt");
    
    let matrix: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let xmas_count: u64 = (0..matrix.len())
        .map(|i| {
            (0..matrix[i].len())
                .filter(|&j| matrix[i][j] == 'X')
                .map(|j| check_xmas(&matrix, i as i32, j as i32))
                .sum::<u64>()
        })
        .sum();

    println!("answer for the first challenge is {}", xmas_count);
}


fn second_challenge() {
    let contents = std::fs::read_to_string("input.txt").expect("fail to read from input.txt");
    let matrix: Vec<Vec<char>> = contents.lines().map(|line| line.chars().collect()).collect();
    let xmas_count: u64 = (0..matrix.len())
        .map(|i| {
            (0..matrix[i].len())
                .filter(|&j| matrix[i][j] == 'A')
                .map(|j| check_x_mas(&matrix, i, j))
                .sum::<u64>()
        })
        .sum();
    
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
