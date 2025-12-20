use std::cmp::{min, max, Ordering};


#[derive(Debug,Copy, Clone, Eq, PartialEq)]
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

    fn overlaps(&self, another: &Self) -> bool{
        another.is_in(self.from) || another.is_in(self.to) 
    }

    fn pairwise_merge(one: &Self, another: &Self) -> Option<Range>{
        if !one.overlaps(another) {
            return None
        }
        else {
            Some(
                Range {
                    from: min(one.from, another.from),
                    to: max(one.to, another.to)

                }
            )
        }
    }
}

impl Ord for Range {
    fn cmp(&self, other: &Self) -> Ordering {
        self.from.cmp(&other.from)
    }
}
impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
fn main() {
    println!("Can it be merged? {}", !Range::pairwise_merge(&Range::new(1,3), &Range::new(2,4)).is_none());
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
        .map(|mut string_array| Range::new(string_array.next().unwrap().parse::<u64>().unwrap(), string_array.next().unwrap().parse::<u64>().unwrap()))
        .collect::<Vec<Range>>();

    range.sort();

    for i in &range[0..10] {
        println!("Before editing:{:?}", i)
    }
    iterative_merge(&mut range);

    for i in &range[0..10] {
        println!("After: {:?}", i)
    }
    println!("Total element: {}",count_total_elements(&range))
}

fn iterative_merge(vec: &mut Vec<Range>) {
    let mut index = 1;
    while index<vec.len() {
        let merge_attempt = Range::pairwise_merge(&vec[index-1],&vec[index]);
        if merge_attempt.is_none() {
            index+=1;
        }
        else {
            vec[index] = merge_attempt.unwrap();
            vec.remove(index-1);
        }
    }
}
fn count_total_elements(vec: &Vec<Range>)->u64 {
    let mut count = 0;
    for range in vec {
        count+=range.to-range.from+1
    }
    count
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
