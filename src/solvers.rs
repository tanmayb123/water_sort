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

pub fn recursive_bfs(state: PuzzleState) -> usize {
    fn _recursive_bfs(state: PuzzleState, move_history: &mut VecDeque<String>, memos: &mut HashMap<PuzzleState, usize>) -> usize {
        if let Some(depth) = memos.get(&state) {
            return *depth;
        }

        if move_history.len() > 100 {
            return 0;
        }

        if state.solved() {
            println!("Found solution at depth: {}", move_history.len());
            memos.insert(state, 1);
            return 1;
        }

        let mut iterator = state.neighbour_iterator();
        let mut min_depth = usize::MAX;
        let mut min_depth_move: Option<String> = None;
        while let Some((neighbour, next_move)) = iterator.next() {
            move_history.push_back(next_move);
            let depth = _recursive_bfs(neighbour, move_history, memos) + 1;
            let next_move = move_history.pop_back();
            if depth < min_depth && depth != 1 {
                min_depth = depth;
                min_depth_move = next_move;
            }
        }

        if min_depth == usize::MAX {
            memos.insert(state, 0);
            return 0;
        }

        memos.insert(state, min_depth);

        return min_depth;
    }

    let mut memos = HashMap::new();
    let mut move_history = VecDeque::new();
    let depth = _recursive_bfs(state, &mut move_history, &mut memos);
    depth
}
