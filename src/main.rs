mod puzzle_state;
mod puzzle_state_neighbour_iterator;
mod puzzle_state_heuristic_calculator;
mod state_heuristic_pair;
mod solvers;

use std::collections::{HashMap, HashSet};

use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

use crate::puzzle_state::PuzzleState;
use crate::solvers::recursive_bfs;

const CONTAINER_SIZE: usize = 4;

fn construct_state_from_colors(colors: Vec<Vec<&str>>) -> PuzzleState {
    let color_idxs = {
        let mut all_colors = HashSet::new();
        for container in &colors {
            for color in container {
                all_colors.insert(color);
            }
        }

        let mut all_colors = all_colors.into_iter().collect::<Vec<_>>();
        all_colors.sort();

        let mut color_idxs: HashMap<&&str, i8> = HashMap::new();
        for (idx, color) in all_colors.into_iter().enumerate() {
            color_idxs.insert(color, (idx + 1) as i8);
        }

        color_idxs
    };

    let mut state = PuzzleState::new(colors.len());

    for (idx, container) in colors.iter().enumerate() {
        let mut container_state = [0; CONTAINER_SIZE];
        for (idx, color) in container.iter().enumerate() {
            container_state[idx] = *color_idxs.get(&color).unwrap();
        }
        state.set_container_states(idx, &container_state);
    }

    state
}

fn my_hash<T>(obj: T) -> u64
where
    T: Hash,
{
    let mut hasher = DefaultHasher::new();
    obj.hash(&mut hasher);
    hasher.finish()
}

fn main() {
    let state = {
        construct_state_from_colors(vec![
            vec!["LG", "OR", "YE", "LB"],
            vec!["DG", "YE", "GR", "LB"],
            vec!["DB", "RE", "LG", "PU"],
            vec!["GY", "RE", "LB", "YE"],
            vec!["YE", "DB", "PI", "DG"],
            vec!["GR", "BR", "DG", "BR"],
            vec!["RE", "PU", "DB", "BR"],
            vec!["BR", "DB", "PI", "PU"],
            vec!["GY", "LG", "PI", "GY"],
            vec!["LB", "GR", "LG", "OR"],
            vec!["RE", "PU", "GY", "DG"],
            vec!["OR", "GR", "OR", "PI"],
            vec![],
            vec![],
        ])
    };

/*
    state1.execute_move(0, 13);
    state1.execute_move(1, 13);

    let state2 = {
        construct_state_from_colors(vec![
            vec!["LG", "OR", "YE"],
            vec!["DG", "YE", "GR"],
            vec!["DB", "RE", "LG", "PU"],
            vec!["GY", "RE", "LB", "YE"],
            vec!["YE", "DB", "PI", "DG"],
            vec!["GR", "BR", "DG", "BR"],
            vec!["RE", "PU", "DB", "BR"],
            vec!["BR", "DB", "PI", "PU"],
            vec!["GY", "LG", "PI", "GY"],
            vec!["LB", "GR", "LG", "OR"],
            vec!["RE", "PU", "GY", "DG"],
            vec!["OR", "GR", "OR", "PI"],
            vec![],
            vec!["LB", "LB"],
        ])
    };

    state1.print();
    println!();
    state2.print();

    let mut m = HashMap::new();
    m.insert(state1, 1);

    let k = m.contains_key(&state2);
    println!("{}", k);
*/

    //state.print();

    let res = recursive_bfs(state);
    println!("Solution: {:?}", res);
    //for item in res {
    //    println!("{:?}", item);
    //}
}
