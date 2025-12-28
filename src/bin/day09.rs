#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Coord {
    x: i32,
    y: i32
}

fn rectangle_size(one_corner: Coord, another_corner: Coord) -> i64 {
    ((one_corner.x-another_corner.x+1) as i64)*((one_corner.y-another_corner.y+1) as i64).abs()
}
fn main() {
//   let input = "7,1
//11,1
//11,7
//9,7
//9,5
//2,5
//2,3
//7,3";
    let input = include_str!("../input/day09.txt");
    part_one(input)
}

fn part_one(input: &str) {
    let coord_list: Vec<Coord> = input.lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| 
            line.split(',')
        ).map(|mut str_array|
            Coord {
                x: str_array.next().unwrap().parse().unwrap(),
                y: str_array.next().unwrap().parse().unwrap(),
            } 
        )
        .collect();

    let mut best: i64 =0;
    for i in 0..coord_list.len() {
        for j in i+1..coord_list.len() {
            let this_rect_size = rectangle_size(coord_list[i], coord_list[j]);
            if this_rect_size > best {
                best = this_rect_size
            }
        }
    }
    println!("The best is {best}");

    
}
