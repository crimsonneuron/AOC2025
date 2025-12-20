use std::collections::HashMap;
fn main() {
    let input = include_str!("../input/day04.txt");
//     let input = "..@@.@@@@.
// @@@.@.@.@@
// @@@@@.@.@@
// @.@@@@..@.
// @@.@@@@.@@
// .@@@@@@@.@
// .@.@.@.@@@
// @.@@@.@@@@
// .@@@@@@@@.
// @.@.@@@.@.";
    let mut map: HashMap<(i32, i32), char> = HashMap::new();
    for (row_index, line) in input.lines().enumerate() {
        for (column_index, char) in line.chars().enumerate() {
            map.insert((column_index as i32, row_index as i32), char);
        }
    }
    //the map is now initialized so that we're basically working exclusively in quadrant IV
    //we're doing <x,y> stuff here exclusvie
    //println!("len is: {}", get_removable(&map).len());
    //println!("Says we can remove: {:?}", get_removable(&map))
    let mut counter = 0;
    let mut removable_targets =get_removable(&map);
    while !removable_targets.is_empty() {
        counter+=removable_targets.len();
        remove(&mut map,removable_targets);
        removable_targets = get_removable(&map);
    }
    println!("Counter says: {counter}");


}

fn count_removable(map: &HashMap<(i32, i32), char>) -> i32 {
    let directions:[(i32, i32);8] = [
    (-1,1),    (0,1),  (1,1),
    (-1, 0),           (1, 0),
    (-1, -1), (0, -1), (1, -1)
    ];

    let mut free_count = 0;
    for element in map {
        let mut roll_count =0;
        if element.1 != &'@' {
            continue
        }
        for direction in directions {
            if map.get(&(((element.0.0 as i32)+direction.0) , ((element.0.1 as i32)+direction.1) )) == Some(&'@') {
                roll_count+=1;
            }
        }
        if roll_count <4 {
            free_count+=1;
        }
    }
    free_count
}
fn get_removable(map: &HashMap<(i32, i32), char>) -> Vec<(i32,i32)> {
    const DELTAS:[(i32, i32);8] = [
    (-1,1),    (0,1),  (1,1),
    (-1, 0),           (1, 0),
    (-1, -1), (0, -1), (1, -1)
    ];
    
    map.iter()
        .filter(|(_, char)| **char=='@')
        .filter(|((x,y), _)|{
            let roll_count = DELTAS.iter()
                .filter(|(dx, dy)| map.get(&(x+dx, y+dy)) == Some(&'@'))
                .count();
            
            roll_count <4
        })
        .map(|((x,y),_)| (*x,*y))
        .collect::<Vec<(i32, i32)>>()
     
}

fn remove(map: &mut HashMap<(i32, i32), char>, targets: Vec<(i32, i32)>) {
    for target in targets {
        map.entry(target).and_modify(|char|*char = '.');
    }
}
