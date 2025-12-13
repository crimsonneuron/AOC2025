fn main() {
    let input = include_str!("../input/day02.txt");
    //let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    let input = input.split(",")
        .map(|range| range.split("-"))
        .map(|mut string_array| string_array.next().expect("Iterator failed").trim().parse::<i64>().expect("First is not a number")..=string_array.next().expect("iterator_failed").trim().parse::<i64>().expect("Second is not a number"))
        .collect::<Vec<_>>();
    
    let mut sum = 0;
    for i in input {
        for j in i {
            if is_invalid(j) {
                sum+=j;
                println!("{j}")
            }
        }
    }
    println!("{sum}");
    println!("{}", is_invalid(123));
    
}

fn is_invalid(n: i64) -> bool {
    let s = n.to_string();
    for chunk_size in 1..=s.len()/2 {
        let first_chunk = &s[0..chunk_size];
        if (chunk_size..s.len()).step_by(chunk_size).all(|i| &s[i..i+chunk_size] ==first_chunk) {
            return true
        }
    }
    false
}

