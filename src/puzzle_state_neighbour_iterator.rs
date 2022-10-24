use crate::puzzle_state::PuzzleState;

pub struct PuzzleStateNeighbourIterator<'a> {
    state: &'a PuzzleState,
    container_from_idx: usize,
    container_to_idx: usize,
    finished: bool,
}

impl<'a> PuzzleStateNeighbourIterator<'a> {
    pub fn new(state: &'a PuzzleState) -> Self {
        Self {
            state,
            container_from_idx: 0,
            container_to_idx: 0,
            finished: false,
        }
    }

    pub fn next(&mut self) -> Option<(PuzzleState, String)> {
        if self.finished {
            return None;
        }

        for from in self.container_from_idx..self.state.container_count() {
            for to in self.container_to_idx..self.state.container_count() {

                if from == to {
                    continue;
                }
                if !self.state.can_move(from, to) {
                    continue;
                }

                let mut new_state = self.state.clone();
                new_state.execute_move(from, to);

                self.container_from_idx = from;
                self.container_to_idx = to + 1;

                return Some((new_state, format!("{} -> {}", from + 1, to + 1)));
            }

            self.container_to_idx = 0;
        }

        self.finished = true;
        None
    }
}
