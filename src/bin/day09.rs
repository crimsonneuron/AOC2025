use std::collections::{HashSet, HashMap};
#[derive(Debug)]
struct Boundaries {
    max_x: i32,
    max_y: i32,
    min_x: i32,
    min_y: i32,
}

impl Boundaries {
    fn index(&self, dir: Direction) -> i32 {
        match dir {
            Direction::XPos => self.max_x,
            Direction::YPos => self.max_y,
            Direction::XNeg => self.min_x,
            Direction::YNeg => self.min_y,
        }
    }
}
#[derive(Debug,Hash, PartialEq, Eq, Copy, Clone)]
struct Coord {
    x: i32,
    y: i32
}
//What is this?
//Uhh, in the direction we're going
//i.e., we'll need a {x,y} to update and a {-1, 1} to update it with
//then while(val less than relevant bounds)
//if in the hashset, we're good
//else return false
impl Coord {
    fn add(&mut self, other: Self) {
        self.x+=other.x;
        self.y+=other.y;
    }
    fn ray_intersects (&self, edges: &HashSet<Coord>, direction: Direction, bounds: &Boundaries) -> bool {
        println!("Checking this coord {:?}, direction {:?}", self, direction);

        let mut ray_coord = self.clone();
        let delta  = match direction {
            Direction::XPos => Coord {x:1, y:0},
            Direction::XNeg => Coord {x:-1, y:0},
            Direction::YPos => Coord {x: 0,y: 1},
            Direction::YNeg => Coord {x: 0,y: -1},
        };
        let outside_bound = bounds.index(direction);
        let mut relevant_coord = ray_coord.get_relevant_coord(direction);
        while relevant_coord!= outside_bound {
            if edges.contains(&ray_coord) {
                println!("This ray check is returning true!");
                return true
            }
            relevant_coord = ray_coord.get_relevant_coord(direction);
            println!("Rel coord is {}", relevant_coord);
            ray_coord.add(delta);
        }
        println!("This check is returning false");
        false
    
    }
    fn get_relevant_coord(&self, dir: Direction) -> i32 {
        match dir {
            Direction::XPos =>  self.x,
            Direction::XNeg =>  self.x,
            Direction::YPos =>  self.y,
            Direction::YNeg =>  self.y,
        }
        
    }
    fn is_coord_valid(&self, edges: &HashSet<Coord>, bounds: &Boundaries) -> bool{
        [Direction::XPos, Direction::YPos, Direction::XNeg, Direction::YNeg].iter().all(|&dir| self.ray_intersects(edges, dir, bounds))
    }

}


#[derive(Debug,Eq, PartialEq,Clone,Copy)]
enum Direction {
    XPos,
    YPos,
    XNeg,
    YNeg
}


#[derive(Eq, PartialEq)]
enum Axis {
    Y,
    X, 
}

impl Axis {
    fn expand_into_dirs(&self) -> [Direction;2] {
        match &self {
            Axis::Y => [Direction::YPos, Direction::YNeg],
            Axis::X => [Direction::XPos, Direction::XNeg],
        }
    }
    fn not(&self) -> Self {
        match &self {
            Axis::X => Axis::Y,
            Axis::Y => Axis::X,
        }
    }
}


#[derive(Copy, Clone, Debug)]
struct Line {
    start_point: Coord,
    end_point: Coord 
}
impl Line {
     fn get_axis(&self) -> Axis {
        if self.start_point.x==self.end_point.x {
            return Axis::Y
        }
        else {
            return Axis::X
        }
    }
  //  fn ray_check_legality(&self, edges: &HashSet<Coord>, bounds: &Boundaries) -> bool {
  //      println!("Checking a line");
  //      let axis = self.get_axis();
  //      let axis_directions = axis.expand_into_dirs();
  //      let off_axis_directions = axis.not().expand_into_dirs();
  //      
  //      //checking the axis that the line is parallel with
  //      //only need to check once, since this'll hold for every coord on the axis
  //      if !self.start_point.ray_intersects(&edges, axis_directions[0], &bounds) || !self.start_point.ray_intersects(&edges, axis_directions[1], &bounds) {
  //          println!("This line failed on the on-axis check");
  //          return false;
  //      }
  //      
  //      //for each coords
  //      //if any coord, on either direction, returns false, return false
  //      let true_false = self.into_iter().all(|coord| {
  //          coord.ray_intersects(&edges, off_axis_directions[0], &bounds) && coord.ray_intersects(&edges, off_axis_directions[1], &bounds)
  //      });
  //      if !true_false {
  //          println!("The line {:?} failed the off axis check", self)
  //      }
  //      true_false
  //  }
}
impl IntoIterator for Line {
    type Item =Coord;
    type IntoIter = std::vec::IntoIter<Coord>;
    fn into_iter(self) -> Self::IntoIter {
        if self.get_axis() == Axis::X {
            (self.start_point.x ..=self.end_point.x).map(|x_coord| Coord{x: x_coord, y:self.start_point.y}).collect::<Vec<Coord>>().into_iter()
        }
        else {
            (self.start_point.y ..=self.end_point.y).map(|y_coord| Coord{x: self.start_point.x, y:y_coord}).collect::<Vec<Coord>>().into_iter()
        }
        
    }
}


struct Rectangle {
    one_corner: Coord,
    another_corner: Coord
}
impl Rectangle {
    fn rectangle_size(&self) -> i64 {
        ((self.one_corner.x-self.another_corner.x+1) as i64)*((self.one_corner.y-self.another_corner.y+1) as i64).abs()
    }
   // fn to_lines(&self) -> [Line;4] {
   //     [
   //         Line {
   //             start_point: self.one_corner, end_point: Coord {x:self.one_corner.x, y:self.another_corner.y}
   //         },
   //         Line {
   //             start_point: Coord {x:self.one_corner.x, y:self.another_corner.y}, end_point: self.another_corner
   //         },
   //         Line {
   //             start_point: self.another_corner, end_point: Coord {x: self.another_corner.x, y: self.one_corner.y}
   //         },
   //         Line {
   //             start_point: Coord {x: self.another_corner.x, y: self.one_corner.y}, end_point: self.one_corner
   //         }
   //     ]
   // }
   // fn is_valid(&self, edges: &HashSet<Coord>, bounds: &Boundaries) -> bool {
   //     let lines = self.to_lines();
   //     lines.into_iter().all(|line| line.ray_check_legality(edges, bounds))
   // }

}
fn rectangle_size(one_corner: Coord, another_corner: Coord) -> i64 {
    ((one_corner.x-another_corner.x+1) as i64)*((one_corner.y-another_corner.y+1) as i64).abs()
}
fn main() {
   let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
    //let input = include_str!("../input/day09.txt");
    part_two(input)
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

//ok we're doing this with ray-casting
//I feel quite stupid for not thinking of this earlier
//For each point on the perimeter (because it must be one curve, so we should always be good there)
//We send rays out in four directions. If all four rays intersect with an edge, we're good
//How do we ensure that we're not sending rays out to infinity?
//uh we could keep track of the furthest-out bounds of the input
//and if it gets there, we call it infinity and say that clearly the point is not in the polygon
//Ok this is going to be not so pretty :(
//We cope and seethe, and then check out the PIP wiki article
//Actually I just need to keep track of (min, max) of (x, y) and compare. 
//neat neat neat
//
//Ok how to implement this legality checking thing 
//Uhh something that takes a line, I guess
//Then on coords we impl ray-casting
//and then just have the func that takes a line do the one x/y cast (just like the extreme value
//for ease )
//
//Ok that turns out to be an O(n^3) and it never returns. New plan: Cells and prefix sums
//For each unique x and each unique y, build the Cells
//in particular, we're going to do that (since the vecs will be in desc order), each cell gets the
//current value, then the next value+1.
//
//
//For the new coords, we'll need to come up with something. I guess that at each 
//Then, we'll need to keep around the coord checks just to check whether each cell is good
//
//Then we'll do a prefix sum 
fn part_two(input: &str) {
    //get the vec of points
    //
    let mut coord_list: Vec<Coord> = Vec::new();
    let mut x_list: Vec<i32> = Vec::new();
    let mut y_list: Vec<i32> = Vec::new();
    
    for line in input.lines().filter(|line| !line.trim().is_empty()) {
        let (x, y) = line.split_once(',').unwrap();
        let (x,y) = (x.trim().parse::<i32>().unwrap(), y.trim().parse::<i32>().unwrap());
        x_list.push(x);
        y_list.push(y);
        coord_list.push(Coord {x: x, y: y});
    }



    let mut edges: HashSet<Coord> = HashSet::new();
    for window in coord_list.windows(2) {
        let line = Line {start_point: window[0], end_point: window[1]};
        for coord in line.into_iter() {
            edges.insert(coord);
        }
    }
    let loop_line = Line {start_point: coord_list[0], end_point: coord_list[coord_list.len()-1]};

    for coord in loop_line {
       edges.insert(coord);
    }

    x_list.sort();
    x_list.dedup();
    y_list.sort();
    y_list.dedup();
    
    let mut x_to_grid: HashMap<i32, usize> = x_list.iter().enumerate().map(|(i, &x)| (x,i)).collect();
    let mut y_to_grid: HashMap<i32, usize> = y_list.iter().enumerate().map(|(i, &y)| (y,i)).collect();

    
    let bounds = Boundaries {max_x: x_list[x_list.len()-1], max_y: y_list[y_list.len()-1], min_y: y_list[0], min_x: x_list[0]};

    
    //validity_Grid[y][x];
    let validity_grid: Vec<Vec<bool>> = y_list.windows(2).map(|[y1,y2]| {
         x_list.windows(2).map(|[x1,x2]|{
            Coord {
                x: (x1+x2)/2,
                y: (y1+y2)/2
            }.is_coord_valid(&edges, &bounds)
        }).collect::<Vec<bool>>()
    }).collect();
        
    let mut prefix_grid: Vec<Vec<i32>>= Vec::new();
    for y_index in 0..validity_grid.len() {
        for x_index in 0..validity_grid[0].len() {
            if y_index ==0 {
                if x_index == 0{
                    prefix_grid[0][0] = validity_grid[0][0] as i32;
                }
                else {
                    prefix_grid[y_index][x_index] =prefix_grid[y_index][x_index-1] as i32;
                }
            }
            else {
                if x_index==0 {
                    prefix_grid[y_index][x_index] = prefix_grid[y_index-1][x_index] + validity_grid[x_index][y_index] as i32
                }
                else {
                    prefix_grid[y_index][x_index] = prefix_grid[y_index-1][x_index] + prefix_grid[y_index][x_index-1] - prefix_grid[x_index-1][y_index-1] + validity_grid[x_index][y_index] as i32
                }
            }
        }

        fn is_rect_valid(prefix_grid: Vec<Vec<i32>>, rect: Rectangle, x_to_grid: HashMap<i32, usize>, y_to_grid: HashMap<i32, usize>) -> bool {
        
            let prefix_sum = prefix_grid[x_cell][y_cell] -
        }
    }


   //Testing ---------------
   
    let mut best: i64 = 0;
    
    for i in 0..coord_list.len() {
        for j in i+1..coord_list.len() {
            let this_rect = Rectangle {one_corner: coord_list[i], another_corner: coord_list[j]};
            if this_rect.is_valid(&edges, &bounds) && this_rect.rectangle_size() > best {
                best = this_rect.rectangle_size();
            }
        }
    }

    println!("The largest rectangle's size is {best}");

}

fn populate_grid(x_map: &HashMap<i32, usize>,x_list: &Vec<i32>, y_map: &HashMap<i32, usize>, y_list: &Vec<i32>, ) -> HashMap<(usize, usize), bool> {
    //over all of the i32s in these maps
    //If the relevant like midpoint is inside of the grid, then we're going to add the usize usize
    //pair as true 
    //neat neat neat
    x_list.windows(2).zip(y_list.windows(2)).map(|([x1, x2], [y1,y2])| {
        Coord{
            x: (x1+x2)/2,
            y: (y1+y2)/2
        }.is_coord_valid
    })
     
}


