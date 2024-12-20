fn first_challenge() {
    use std::fs;

    // Read the entire file into a string
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    let (mut first_vector, mut second_vector): (Vec<i32>, Vec<i32>) = contents.lines().map(|line| {
        let mut nums = line.split_whitespace().map(|num| num.parse::<i32>().unwrap());
        (nums.next().unwrap(), nums.next().unwrap())
    }).unzip();

    first_vector.sort();
    second_vector.sort();

    let ans_first:i32 = first_vector.iter().zip(second_vector.iter()).map(|(a,b)| (a-b).abs()).sum();

    println!("ans_first: {}", ans_first);
}

fn second_challenge(){
	use std::fs;
    use std::collections::HashMap;
	let contents = fs::read_to_string("input.txt").expect("fail to read from input.txt");
	
	let (first_vector, second_vector): (Vec<i32>, Vec<i32>) = contents.lines().map(|line| {
		let mut nums = line.split_whitespace().map(|num| num.parse::<i32>().unwrap());
		(nums.next().unwrap(), nums.next().unwrap())
	}).unzip();

	let second_frequency: HashMap<i32,i32> = second_vector.iter().fold(HashMap::new(), |mut map, &num| {
		*map.entry(num).or_insert(0)+=1;
		map });

	let ans: i32 = first_vector.iter().map(|num| { second_frequency.get(num).unwrap_or(&0)*num }).sum();

	println!("answer of the second challeng is: {:?}", ans);
}

fn main() {
    first_challenge();
    second_challenge();
}
