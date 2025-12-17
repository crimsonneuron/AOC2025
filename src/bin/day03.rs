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

fn find_biggest_battery(vec: Vec<u32>) -> u64 {
    let mut values= [0;12];
    let mut last_index:isize = -1;
    for i in (1..=12).rev() {
        for j in last_index+1..((vec.len()-(i-1)) as isize) {
            if vec[j as usize] >values[12-i] {
                values[12-i] = vec[j as usize];
                last_index = j;
            }
        }
    }

    values.into_iter().fold(0, |acc, x| acc*10+(x as u64))


}


//for each digit placement (12 through 1), we go through the vector till there are that number
//of digits left -1 (because we always need some more digits afterwards), and we keep track of the
//biggest index found thus far. we mark it down
//current problem is: it is always comparing to index 0, which might just be 9 (i.e., it'll never
//update). 
//So how do I solve this? just another if-statement? i.e., if index ==0, just compare to 0?
//what I want to do here is very simple: go through the vector of the relevant length and for each
//one, find the biggest value.
//The only problem is carrying the things through to the next repetition, because the next search
//should start from where the previous search ended.
//Ok, so just keep both values (instead of this inanity)
