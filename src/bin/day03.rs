fn main() {
    let input = include_str!("../input/day03.txt");
    //let input = "987654321111111\n811111111111119\n234234234234278\n818181911112111";
    let input = input.lines().filter(
        |line| !line.trim().is_empty() //gotta filter out an empty line in the input
    ).map(
        |line| line.chars().map(
            |char| char.to_digit(10).expect("Non-digit in my digits?")
        ).collect::<Vec<u32>>()
    );

    let mut sum = 0;
    for line in input {
        let x = find_biggest_battery(line);
        sum+=x;
        println!("biggest_battery: {x}");
    }
    println!("{sum}");
}

fn find_biggest_battery(vec: Vec<u32>) -> u32 {
    let mut x_1_index = 0;
    for i in 0..vec.len()-1 {
        if vec[i]>vec[x_1_index] {
            x_1_index = i
        }
    }
    let mut x_2_index = x_1_index+1;
    for j in x_1_index+1..vec.len() {
        if vec[j]>vec[x_2_index] {
            x_2_index =j 
        }
    }
    vec[x_1_index]*10 + vec[x_2_index]
}
