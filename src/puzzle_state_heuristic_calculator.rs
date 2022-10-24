use std::collections::HashMap;

use crate::CONTAINER_SIZE;
use crate::puzzle_state::{BlockState, PuzzleState};
use crate::state_heuristic_pair::StateHeuristicPair;

pub struct PuzzleStateHeuristicCalculator {
    memos: HashMap<PuzzleState, i64>,
}

impl PuzzleStateHeuristicCalculator {
    pub fn new() -> Self {
        Self {
            memos: HashMap::new(),
        }
    }

    fn container_heuristic(container: &[BlockState; CONTAINER_SIZE]) -> i64 {
        if container[0] == BlockState::Empty {
            return 0;
        }

        let base_count = {
            let base = container[0];
            let mut count = 1;
            for i in 1..CONTAINER_SIZE {
                if container[i] == base {
                    count += 1;
                }
            }
            count
        };
        
        let remaining_states = {
            let mut remaining = 0;
            for i in base_count..CONTAINER_SIZE {
                if container[i] == BlockState::Empty {
                    remaining = CONTAINER_SIZE - i;
                    break;
                }
            }
            remaining
        };

        (base_count as i64) - (remaining_states as i64)
    }

    fn puzzle_heuristic(state: &PuzzleState) -> i64 {
        (0..state.container_count())
            .map(|x| Self::container_heuristic(&state.get_container(x)))
            .sum()
    }

    pub fn compute_heuristic(&mut self, state: PuzzleState) -> StateHeuristicPair {
        if let Some(heuristic) = self.memos.get(&state) {
            return StateHeuristicPair::new(state, *heuristic);
        }

        let heuristic = Self::puzzle_heuristic(&state);

        self.memos.insert(state.clone(), heuristic);

        StateHeuristicPair::new(state, heuristic)
    }
}
