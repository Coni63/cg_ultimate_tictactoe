use crate::miniboard::{MiniBoard, Player};
use std::{
    collections::HashMap,
    fmt::Debug,
    hash::{Hash, Hasher},
};

pub struct Board {
    sub_boards: [usize; 9],
    main_board: usize,
    previous_grid: usize,
    previous_position: usize,
}

impl Board {
    pub fn new() -> Board {
        Board {
            sub_boards: [0; 9],
            main_board: 0,
            previous_grid: 0,
            previous_position: 0,
        }
    }

    pub fn play(
        &mut self,
        player: Player,
        grid: usize,
        position: usize,
        g: &HashMap<usize, MiniBoard>,
    ) {
        let sub_board_id = self.sub_boards[grid];
        let sub_board = g.get(&sub_board_id).unwrap();
        let new_sub_board = sub_board.get_child(player, position);
        self.sub_boards[grid] = new_sub_board;
        if let Some(player) = sub_board.get_winner() {
            let main_board = g.get(&sub_board_id).unwrap();
            let new_main_board = main_board.get_child(player, position);
            self.main_board = new_main_board;
        }
        self.previous_grid = grid;
        self.previous_position = position;
    }

    pub fn get_possible_actions(&self, g: &HashMap<usize, MiniBoard>) -> Vec<(usize, usize)> {
        let sub_board_index = self.sub_boards[self.previous_position];
        let played_board = g.get(&sub_board_index).unwrap();
        if !played_board.is_over() {
            return played_board
                .get_possible_actions()
                .iter()
                .map(|x| (self.previous_position, *x))
                .collect();
        }

        let mut ans: Vec<(usize, usize)> = Vec::new();
        for (k, sub_board_idx) in self.sub_boards.iter().enumerate() {
            let sub_board = g.get(sub_board_idx).unwrap();
            if !sub_board.is_over() {
                let actions: Vec<(usize, usize)> = sub_board
                    .get_possible_actions()
                    .iter()
                    .map(|x| (k, *x))
                    .collect();
                ans.extend(actions);
            }
        }
        ans
    }
}

impl Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Board:")?;
        writeln!(f, "  Sub_boards: {:?}", self.sub_boards)?;
        writeln!(f, "  Main_board: {}", self.main_board)?;
        Ok(())
    }
}

impl Hash for Board {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.sub_boards.hash(state);
        self.main_board.hash(state);
    }
}

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        self.sub_boards == other.sub_boards && self.main_board == other.main_board
    }
}

impl Eq for Board {}
