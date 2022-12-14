use std::hash::{Hash, Hasher};

use crate::CONTAINER_SIZE;
use crate::puzzle_state_neighbour_iterator::PuzzleStateNeighbourIterator;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum BlockState {
    Empty,
    KnownColor(i8),
    // TODO: UnknownColor,
}

#[derive(Clone)]
pub struct PuzzleState {
    containers: Vec<[BlockState; CONTAINER_SIZE]>,
    //move_history: Vec<Option<usize>>,
}

impl PuzzleState {
    pub fn new(container_count: usize) -> Self {
        Self {
            containers: vec![[BlockState::Empty; CONTAINER_SIZE]; container_count],
            //move_history: vec![None; container_count],
        }
    }

    pub fn set_container_states(&mut self, container_idx: usize, states: &[i8; CONTAINER_SIZE]) {
        for (idx, state) in states.iter().enumerate() {
            let state = *state;
            self.containers[container_idx][idx] = {
                if state == -1 {
                    unreachable!()
                    // TODO: BlockState::UnknownColor
                } else if state == 0 {
                    BlockState::Empty
                } else {
                    BlockState::KnownColor(state)
                }
            }
        }
    }

    fn container_bottom_empty_idx(&self, container_idx: usize) -> Option<usize> {
        if self.containers[container_idx][CONTAINER_SIZE - 1] != BlockState::Empty {
            return None;
        }
        for i in 0..CONTAINER_SIZE {
            if self.containers[container_idx][i] == BlockState::Empty {
                return Some(i);
            }
        }
        unreachable!();
    }

    fn container_top_state(&self, container_idx: usize) -> (BlockState, usize) {
        let mut i = CONTAINER_SIZE - 1;
        while i > 0 && self.containers[container_idx][i] == BlockState::Empty {
            i -= 1;
        }
        (self.containers[container_idx][i], i)
    }

    pub fn can_move(&self, from: usize, to: usize) -> bool {
        if from == to {
            return false;
        }

        let (from_state, _) = self.container_top_state(from);
        let (to_state, _) = self.container_top_state(to);
        let to_full = self.container_bottom_empty_idx(to);

        //if let Some(from_history) = self.move_history[from] {
        //    if from_history == to {
        //        //return false;
        //    }
        //}

        return
            !(to_full == None) &&
            from_state != BlockState::Empty &&
            (to_state == BlockState::Empty || from_state == to_state);
    }

    fn remove_from_container(&mut self, container_idx: usize, max: usize) -> (BlockState, usize) {
        let (top_state, idx) = self.container_top_state(container_idx);
        assert!(top_state != BlockState::Empty);

        let mut state_count = 0;
        for i in (0..idx + 1).rev() {
            if self.containers[container_idx][i] != top_state || state_count == max {
                break;
            }
            self.containers[container_idx][i] = BlockState::Empty;
            state_count += 1;
        }

        (top_state, state_count)
    }

    pub fn execute_move(&mut self, from: usize, to: usize) -> bool {
        assert!(self.can_move(from, to));

        let to_idx: usize;
        match self.container_bottom_empty_idx(to) {
            Some(idx) => to_idx = idx,
            None => return false
        }

        let to_capacity = CONTAINER_SIZE - to_idx;
        let (from_state, count) = self.remove_from_container(from, to_capacity);
        for i in to_idx..to_idx + count {
            self.containers[to][i] = from_state.clone();
        }

        /*
        if let Some(from_last_move) = self.move_history[from] {
            self.move_history[from_last_move] = None;
        }
        self.move_history[from] = Some(to);
        if let Some(to_last_move) = self.move_history[to] {
            self.move_history[to_last_move] = None;
        }
        self.move_history[to] = Some(from);
        */

        true
    }

    pub fn neighbour_iterator<'a>(&'a self) -> PuzzleStateNeighbourIterator<'a> {
        PuzzleStateNeighbourIterator::new(&self)
    }

    fn container_solved(&self, container_idx: usize) -> bool {
        if self.containers[container_idx][0] == BlockState::Empty {
            return true;
        }

        for i in 1..CONTAINER_SIZE {
            if self.containers[container_idx][i] != self.containers[container_idx][i - 1] {
                return false;
            }
        }

        true
    }

    pub fn solved(&self) -> bool {
        !(0..self.containers.len())
            .any(|x| !self.container_solved(x))
    }
    
    pub fn container_count(&self) -> usize {
        self.containers.len()
    }

    fn hash_container(container: &[BlockState; CONTAINER_SIZE]) -> u64 {
        let mut hash = 0;
        for block in container {
            let val = match block {
                BlockState::Empty => 0,
                BlockState::KnownColor(color) => *color as u64,
            };
            hash += val;
            hash <<= 4;
        }
        hash
    }

    pub fn hashed_containers(&self) -> Vec<u64> {
        self.containers
            .iter()
            .map(|x| Self::hash_container(x))
            .collect()
    }
}

impl PartialEq for PuzzleState {
    fn eq(&self, other: &Self) -> bool {
        let mut self_hashed = self.hashed_containers();
        self_hashed.sort();
        let mut other_hashed = other.hashed_containers();
        other_hashed.sort();
        self_hashed == other_hashed
    }
}

impl Eq for PuzzleState {}

impl Hash for PuzzleState {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut hashed = self.hashed_containers();
        hashed.sort();
        hashed.hash(state);
    }
}
