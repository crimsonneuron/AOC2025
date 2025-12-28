fn main() {
    let input = include_str!("../input/day07.txt");
//    let input = ".......S.......
//...............
//.......^.......
//...............
//......^.^......
//...............
//.....^.^.^.....
//...............
//....^.^...^....
//...............
//...^.^...^.^...
//...............
//..^...^.....^..
//...............
//.^.^.^.^.^...^.
//...............";
    part_two(input)
}
fn part_two(input: &str) {
    const WIDTH:usize= 141;
    type State = [i64; WIDTH];
    let splitter_pos: Vec<[bool; WIDTH]> = input.lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.chars().map(
            |char| char=='^'
        ).collect::<Vec<_>>().try_into().expect("some problem"))
        .collect();

    fn update_line(old_state: State, splitter_poses: [bool;WIDTH]) -> State {
        let mut return_state = [0;WIDTH];
        for (index,laser) in old_state.into_iter().enumerate() {
            if laser ==0 {
                continue
            }
            if splitter_poses[index] {
                return_state[index-1]+=laser;
                return_state[index+1]+=laser;
            }
            else {
                return_state[index]+=laser
            }
        }
        return_state 
    }

    let mut old_state: State = input.lines().next().unwrap().chars().map(|char| 
        match char {
            'S'=>1,
            _=>0
        }
    ).collect::<Vec<_>>().try_into().unwrap();

    for i in 1..splitter_pos.len() {
        old_state = update_line(old_state, splitter_pos[i]);
    }
    let total_count:i64 = old_state.into_iter().sum();
    println!("The total number of parallel universes is {}", total_count);
    
}
fn part_one(input: &str) {

#[derive(Copy, Clone, Debug)]
    enum Cell {
        Empty,
        Beam,
        Splitter
    }

    type State = [Cell; 141];
    
    fn update_state(old_state: State, current_state: State) -> (State, i32) {
        let mut return_state = [Cell::Empty;141];
        let mut split_counter = 0;
        for i in 0..current_state.len() {
            match old_state[i] {
                Cell::Empty => (),
                Cell::Beam => {
                   match current_state[i] {
                        Cell::Empty => return_state[i] = Cell::Beam,
                        Cell::Beam => return_state[i] = Cell::Beam,
                        Cell::Splitter => {return_state[i-1] = Cell::Beam; return_state[i+1] = Cell::Beam; split_counter+=1},
                    } 
                },
                Cell::Splitter => (),
            }
        }
        (return_state, split_counter)
    }
       let splitter_states: Vec<State> = input.lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.chars().map(
            |char| match char {
                '|'|'S'=> Cell::Beam,
                '.'=> Cell::Empty,
                '^'=> Cell::Splitter,
                _=> panic!("Some wrong char in input")
            }).collect::<Vec<_>>().try_into().expect("problem line, in particular {}")
        ).collect();

    let mut old_state = splitter_states[0];
    let mut split_counter = 0;
    for i in 1..splitter_states.len() {
        let update = update_state(old_state, splitter_states[i]);
        old_state = update.0;
        split_counter+=update.1
    }
    println!("Number of splits is {split_counter}");


}
//Update state just takes an array full of just lasers
//And accordingly returns just lasers, so that it can be easily chained together

//it's basically a cellular automata working downwards
//the only things relevant for this thing are the previous row and the locations of the splitters
//on the current row
//

