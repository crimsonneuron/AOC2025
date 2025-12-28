enum Op {
    Mul,
    Add,
}
struct Problem {
    values: Vec<i64>,
    op: Op
}

impl Problem {
    fn return_val(self) -> i64 {
        match self.op {
            Op::Mul => {self.values.into_iter().reduce(|acc, val| acc*val).expect("mul failed p2")},
            Op::Add => {self.values.into_iter().reduce(|acc, val| acc+val).expect("Add failed p2")}

        }
    }
    fn new(vec: Vec<[char;5]>) -> Self {
        let operator = match vec[0][4] {
            '+' => Op::Add,
            '*' => Op::Mul,
            _ => panic!("Tried to match on a char operator neither * nor +, specificall {}", vec[0][3]),
        };
        let values = vec.iter()
            .map(|str_arr| 
                str_arr.iter().filter(|char| char.is_digit(10))
                .collect::<String>()
                .parse::<i64>().expect("All of these digits not an int?")
            )
            .collect::<Vec<i64>>();
        Problem {
            values: values,
            op:operator
        }
    }
}

fn main() {
    let input = include_str!("../input/day06.txt");
//    let input =
//"123 328  51 64 
// 45 64  387 23 
//  6 98  215 314
//*   +   *   +  ";
    part_two(input);
}


fn part_two(input: &str) {
    let twodcharvec = input.lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut vertical_slices = Vec::<[char;5]>::with_capacity(twodcharvec[0].len());

    for i in 0..twodcharvec[0].len() {
        let arr = [twodcharvec[0][i],twodcharvec[1][i],twodcharvec[2][i],twodcharvec[3][i], twodcharvec[4][i]];
        vertical_slices.push(arr)
    }

    let sum = vertical_slices.split(|&x| x==[' ';5])
        .map(|slice| slice.to_vec())
        .map(|problem_slice| Problem::new(problem_slice))
        .fold(0, |acc, val| acc+val.return_val());
    println!("The final value is: {sum}");

     
}


fn part_one(input: String) {

    let numbers = input.lines()
        .take(4)
        .map(|line| line.split_whitespace())
        .map(|str_arr| str_arr.map(|element| element.parse::<i64>().expect("Not a number")))
        .map(|line| line.collect::<Vec<i64>>())
        .collect::<Vec<Vec<i64>>>();
    let operators = input.lines()
        .nth(4).expect("There are 4 lines in the input")
        .chars()
        .filter(|char| !char.is_whitespace())
        .collect::<Vec<char>>();
    
    let mut counter:i64 = 0;
    for (i, op) in operators.into_iter().enumerate() {
        let arr = [numbers[0][i],numbers[1][i],numbers[2][i], numbers[3][i]];
        counter+=do_column(arr, op)
    }

    println!("Final result: {counter}")

}
fn do_column(arr:[i64;4], op:char)->i64 {
    match op {
        '+' => {return arr.into_iter().reduce(|acc, val| acc+val).expect("Adding failed")},
        '*' => {return arr.into_iter().reduce(|acc, val| acc*val).expect("multiplying failed")},
        _ => {println!("The crashing operator was {op}"); panic!()} 

    }
}
