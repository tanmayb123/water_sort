use std::collections::{VecDeque, HashMap};

use crate::puzzle_state::PuzzleState;

pub fn recursive_bfs(state: PuzzleState) -> Option<Vec<String>> {
    fn _recursive_bfs(state: PuzzleState, move_history: &mut VecDeque<String>, memos: &mut HashMap<PuzzleState, usize>, best_solution: &mut Box<Option<Vec<String>>>) -> usize {
        if state.solved() {
            let solution = move_history.iter().cloned().collect::<Vec<_>>();
            if best_solution.is_none() || best_solution.as_ref().as_ref().unwrap().len() > solution.len() {
                **best_solution = Some(solution);
            }
            memos.insert(state, 1);
            return 1;
        }

        if let Some(depth) = memos.get(&state) {
            return *depth;
        }

        if move_history.len() > 100 {
            return 0;
        }

        let mut iterator = state.neighbour_iterator();
        let mut min_depth = usize::MAX;
        while let Some((neighbour, next_move)) = iterator.next() {
            move_history.push_back(next_move);
            let depth = _recursive_bfs(neighbour, move_history, memos, best_solution) + 1;
            _ = move_history.pop_back();
            if depth < min_depth && depth != 1 {
                min_depth = depth;
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
    let mut best_solution = Box::new(None);
    _ = _recursive_bfs(state, &mut move_history, &mut memos, &mut best_solution);
    *best_solution
}
