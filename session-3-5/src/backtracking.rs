use std::vec;

use crate::{journey::{Journey, Solution}, position::Position};

pub struct Solutions {
    possible_moves: Vec<Position>,
    open: Vec<Candidate>,
    current: Journey,
    count: i32,
}

impl Solutions {
    pub fn new(start: Position) -> Self {
        Self {
            possible_moves: Vec::new(),
            open: vec![Candidate{ count: 0, mov: start }],
            current: Journey::new(start),
            count: 0,
        }
    }
}

struct Candidate {
    /// Counts the number of turns made to get to this candidate. We keep track of this so we can
    /// call undo the appropriate number of types, if we roll back a solution.
    count: i32,
    mov: Position,
}

impl Iterator for Solutions {
    type Item = Solution;

    fn next(&mut self) -> Option<Self::Item> {

        while let Some(Candidate { count, mov} ) = self.open.pop() {
            // Unroll all the moves until our current state is identical with the one which we
            // had once we put that mov into the open list. We want to be one move behind so
            // we need to play the move in order to get the desired state
            for _ in 0.. self.count - count + 1{
                self.current.undo()
            }

            // We advance one move deeper into the search tree
            self.current.play_move(mov);
            self.count = count;

            // Emit solution
            if let Some(solution) = self.current.is_solution() {
                return Some(solution);
            }

            // Extend search tree
            self.current.fill_possible_moves(&mut self.possible_moves);
            self.open
                .extend(self.possible_moves.iter().map(|&position| {
                    Candidate { count: count + 1, mov: position}
                }))
        }
        None
    }
}
