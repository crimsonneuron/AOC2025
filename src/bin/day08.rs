use std::collections::BinaryHeap;
use std::cmp::{Ordering,Reverse};

#[derive(Debug)]
struct Node {
    x: i32, 
    y: i32,
    z: i32,
    neighbors: Vec<usize>
}

impl Node {
    fn distance(&self, another: &Self) -> f64 {
        let dx = (self.x-another.x) as f64;
        let dy = (self.y-another.y) as f64;
        let dz = (self.z-another.z) as f64;
        (dy.powf(2.0)+dx.powf(2.0)+dz.powf(2.0)).sqrt()
    }
}
#[derive(Copy, Clone, PartialEq,Debug)]
struct Pair {
    node_1: usize,
    node_2: usize,
    distance: f64
}

impl Eq for Pair {}
impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}
impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.partial_cmp(&other.distance).unwrap()
    }
}
fn main() {
//    let input = "162,817,812
//57,618,57
//906,360,560
//592,479,940
//352,342,300
//466,668,158
//542,29,236
//431,825,988
//739,650,466
//52,470,668
//216,146,977
//819,987,18
//117,168,530
//805,96,715
//346,949,466
//970,615,88
//941,993,340
//862,61,35
//984,92,344
//425,690,689";
    let input = include_str!("../input/day08.txt");
    part_two(input)
}

fn part_one(input: &str) {
    let mut master_list = input.lines()
        .map(|line| 
            line.split(',').
                map(|number| number.parse::<i32>().unwrap())
        )
        .map(|mut line_iter| 
            Node {
                x: line_iter.next().unwrap(),
                y: line_iter.next().unwrap(),
                z: line_iter.next().unwrap(),
                neighbors: Vec::new(),
            }
        ).collect::<Vec<Node>>();

    fn connect_boxes(list: &mut Vec<Node>) {
        //may need to do this with a min-heap or some such
        //Just really over-engineer the shit out of this
        let mut top_thousand = BinaryHeap::new();
        let n=1000;

        for i in 0..list.len() {
            for j in i+1..list.len() {
                let this_pair = Pair {
                    node_1: i,
                    node_2: j,
                    distance: list[i].distance(&list[j]),
                };
                if top_thousand.len()<n {
                    top_thousand.push(this_pair)
                }
                else if this_pair < *top_thousand.peek().unwrap() {
                    top_thousand.pop();
                    top_thousand.push(this_pair)
                }
            }
        }

        
        let mut pairs_vec: Vec<_> = top_thousand.into_iter().collect();
        pairs_vec.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());
        for pair in pairs_vec{
            list[pair.node_1].neighbors.push(pair.node_2);
            list[pair.node_2].neighbors.push(pair.node_1);
        }
    }

    connect_boxes(&mut master_list);
    fn explore_graph(start: usize, list: &Vec<Node>) -> Vec<usize> {
        let mut visited: Vec<usize> =Vec::new();
        let mut to_visit: Vec<usize> = vec![start];
        while !to_visit.is_empty() {
            let current_node = to_visit.pop().unwrap();
            let mut children_list:Vec<usize> = list[current_node].neighbors.iter().filter(|&child|
                !visited.iter().any(|&node| node==*child) && !to_visit.iter().any(|&node| node==*child)
            )
                .map(|&index| index)
                .collect();
            to_visit.append(&mut children_list);
            visited.push(current_node);
        }
        visited
    }

    let mut visited:Vec<usize> = Vec::new();
    let mut top_three: Vec<usize> = Vec::new();
    for (index,node) in master_list.iter().enumerate() {
        if node.neighbors.is_empty() {
            continue
        } 
        if visited.iter().any(|&visited_node| visited_node==index) {
            continue
        }
        let mut sub_graph =explore_graph(index, &master_list);
        top_three.push(sub_graph.len());


        visited.append(&mut sub_graph)
    }
    top_three.sort();
    top_three = top_three[(top_three.len()-3)..].to_vec();
    
    let three_product = top_three.into_iter().reduce(|acc, val| acc*val).unwrap();
    println!("The product of the size of the three longest circuits is {three_product}");
}

fn part_two(input: &str) {
    let mut master_list = input.lines()
        .map(|line| 
            line.split(',').
                map(|number| number.parse::<i32>().unwrap())
        )
        .map(|mut line_iter|
            Node {
                x: line_iter.next().unwrap(),
                y: line_iter.next().unwrap(),
                z: line_iter.next().unwrap(),
                neighbors: Vec::new(),
            }
        ).collect::<Vec<Node>>();
    
    fn connect_boxes(list: &Vec<Node>) -> Vec<Pair>{
        let mut return_list = Vec::new();
        for i in 0..list.len() {
            for j in i+1..list.len() {
                return_list.push(
                    Pair {
                        node_1: i,
                        node_2: j,
                        distance: list[i].distance(&list[j])
                    }
                )
            }
        }
        return_list.sort();
        return_list.reverse();
        return_list
    }
    let mut ordered_connections = connect_boxes(&master_list);

    fn explore_graph(start: usize, list: &Vec<Node>) -> usize{
        let mut visited: Vec<usize> =Vec::new();
        let mut to_visit: Vec<usize> = vec![start];
        while !to_visit.is_empty() {
            let current_node = to_visit.pop().unwrap();
            let mut children_list:Vec<usize> = list[current_node].neighbors.iter().filter(|&child|
                !visited.iter().any(|&node| node==*child) && !to_visit.iter().any(|&node| node==*child)
            )
                .map(|&index| index)
                .collect();
            to_visit.append(&mut children_list);
            visited.push(current_node);
        }
        visited.len()
    }
    let mut do_while = true;

    let mut pair = ordered_connections.pop().unwrap();
    while do_while {
        master_list[pair.node_1].neighbors.push(pair.node_2);
        master_list[pair.node_2].neighbors.push(pair.node_1);
        do_while = explore_graph(0, &master_list)!=master_list.len();
        pair= ordered_connections.pop().unwrap();
    }

        println!("the two x-coords multiplied is {}", master_list[pair.node_1].x as i64 * master_list[pair.node_2].x as i64);

}
