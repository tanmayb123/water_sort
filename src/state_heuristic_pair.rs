use std::cmp::Ordering;

use crate::puzzle_state::PuzzleState;

#[derive(Eq, PartialEq)]
pub struct StateHeuristicPair {
    state: PuzzleState,
    heuristic: i64,
}

impl StateHeuristicPair {
    pub fn new(state: PuzzleState, heuristic: i64) -> Self {
        Self {
            state,
            heuristic
        }
    }

    pub fn get_state(self) -> PuzzleState {
        self.state
    }
}

impl Ord for StateHeuristicPair {
    fn cmp(&self, other: &Self) -> Ordering {
        self.heuristic.cmp(&other.heuristic)
    }
}

impl PartialOrd for StateHeuristicPair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
