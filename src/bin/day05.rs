#[derive(Copy, Clone)]
struct Range {
    from: u64,
    to: u64,
}
impl Range {
    fn is_in(&self, n: u64) -> bool {
        self.from<=n && n<=self.to
    }
    fn new(a: u64, b: u64) -> Range{
        Range{
            from:a,
            to: b,
        }
    }
}
fn main() {
    part_two()
}

fn part_two() {
//    let input = "3-5
//10-14
//16-20
//12-18";
   let input = include_str!("../input/day05.txt").split("\n\n").next().expect(".next failed");
    let mut range = input.lines()
        .map(|line| line.split("-"))
        .flat_map(|mut string_array| string_array.next().unwrap().parse::<u64>().expect("Couldn't parse")..=string_array.next().unwrap().parse::<u64>().expect("Could not parse"))
        .collect::<Vec<u64>>();

    range.sort();
    range.dedup();
    println!("Count: {}",range.len())

}
fn part_one() {
    let mut input = include_str!("../input/day05.txt").split("\n\n");
//    let mut input = "3-5
//10-14
//16-20
//12-18
//
//1
//5
//8
//11
//17
//32".split("\n\n");
    let (ranges, values) = (input.next().expect("Ranges failed"), input.next().expect("Values failed"));

    let ranges = ranges.lines()
        .map(|line| line.split("-"))
        .map(|mut array_iter| Range::new(
            array_iter.next().expect("could not step through iter").parse::<u64>().expect("could not parse"), array_iter.next().expect("could not step through iter").parse::<u64>().expect("could not parse")
        ))
        .collect::<Vec<Range>>();
    let values = values
        .lines()
        .map(|line| line.parse::<u64>().expect("Value could not be parsed"))
        .collect::<Vec<u64>>();

    let count = values.iter().filter(|&&value| is_spoiled(value, ranges.clone())).count();
    println!("count: {count}")
}

fn is_spoiled(ingredient: u64, ranges:Vec<Range>) -> bool {
    ranges.iter().any(|range| range.is_in(ingredient)) 
}
