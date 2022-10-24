use std::collections::{VecDeque, HashMap, HashSet};

use crate::puzzle_state::PuzzleState;
use crate::puzzle_state_heuristic_calculator::PuzzleStateHeuristicCalculator;
use crate::state_heuristic_pair::StateHeuristicPair;

pub fn dfs(state: PuzzleState) -> Option<Vec<String>> {
    fn _dfs(state: PuzzleState, prev_move: String, memos: &mut HashSet<PuzzleState>, solution: &mut Vec<String>, depth: usize) -> bool {
        if memos.contains(&state) {
            return false;
        } else {
            memos.insert(state.clone());
        }

        if state.solved() {
            solution.reserve(depth);
            solution.push(prev_move);
            return true;
        }

        let mut iterator = state.neighbour_iterator();
        while let Some((neighbour, next_move)) = iterator.next() {
            if _dfs(neighbour, next_move, memos, solution, depth + 1) {
                solution.push(prev_move);
                return true;
            }
        }

        false
    }

    let mut memos = HashSet::new();
    let mut solution = Vec::new();
    if !_dfs(state, "".to_string(), &mut memos, &mut solution, 1) {
        None
    } else {
        Some(
            solution
                .into_iter()
                .rev()
                .collect()
        )
    }
}

pub fn bfs(state: PuzzleState) -> bool {
    let mut to_explore = VecDeque::new();
    let mut explored = HashSet::new();

    to_explore.push_back(state);

    while let Some(state) = to_explore.pop_front() {
        if explored.contains(&state) {
            continue;
        }

        explored.insert(state.clone());

        let mut iterator = state.neighbour_iterator();
        while let Some((neighbour, _)) = iterator.next() {
            if neighbour.solved() {
                return true;
            }

            to_explore.push_back(neighbour);
        }

        print!("{}\r", explored.len());
    }

    false
}
