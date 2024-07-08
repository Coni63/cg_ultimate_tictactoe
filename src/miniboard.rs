use std::fmt::Debug;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Player {
    X,
    O,
    Free,
    Draw,
}

pub struct MiniBoard {
    cells: [Player; 9],
    winner: Option<Player>,
    childs: [usize; 27],
    _hash: usize,
    _actions: Vec<usize>,
}

impl MiniBoard {
    pub fn new() -> MiniBoard {
        let mut ans = MiniBoard {
            cells: [Player::Free; 9],
            winner: None,
            childs: [0; 27],
            _hash: 0,
            _actions: Vec::new(),
        };
        ans.set_possible_actions();
        ans
    }

    pub fn play(&mut self, player: Player, pos: usize) -> Option<MiniBoard> {
        if self.cells[pos] == Player::Free {
            let mut ans = self.clone();
            ans.cells[pos] = player;
            ans.set_winner();
            ans.set_hash();
            ans.set_possible_actions();
            self.set_child(player, pos, ans.get_hash());
            return Some(ans);
        }
        None
    }

    pub fn set_child(&mut self, player: Player, pos: usize, child: usize) {
        let offset = match player {
            Player::X => 0,
            Player::O => 9,
            Player::Draw => 18,
            _ => 0,
        };
        self.childs[offset + pos] = child;
    }

    pub fn set_hash(&mut self) {
        self._hash = self.hash() as usize;
    }

    pub fn get_hash(&self) -> usize {
        self._hash
    }

    pub fn get_child(&self, player: Player, pos: usize) -> usize {
        let offset = match player {
            Player::X => 0,
            Player::O => 9,
            Player::Draw => 18,
            _ => 0,
        };
        self.childs[offset + pos]
    }

    pub fn set_possible_actions(&mut self) {
        if !self.is_over() && self._actions.is_empty() {
            for i in 0..9 {
                if self.cells[i] == Player::Free {
                    self._actions.push(i);
                }
            }
        }
    }

    pub fn get_possible_actions(&self) -> Vec<usize> {
        self._actions.clone()
    }

    pub fn hash(&self) -> u64 {
        let mut ans = 0;
        for i in 0..9 {
            let val = match self.cells[i] {
                Player::X => 1,
                Player::O => 2,
                Player::Draw => 3,
                _ => 0,
            };
            ans = ans * 3 + val;
        }
        ans
    }

    pub fn is_over(&self) -> bool {
        self.winner.is_some()
    }

    fn set_winner(&mut self) {
        // check columns
        for i in 0..3 {
            if self.cells[i] != Player::Free
                && self.cells[i] == self.cells[i + 3]
                && self.cells[i + 3] == self.cells[i + 6]
            {
                self.winner = Some(self.cells[i]);
                return;
            }
        }

        // check rows
        for i in 0..3 {
            let k = i * 3;
            if self.cells[k] != Player::Free
                && self.cells[k] == self.cells[k + 1]
                && self.cells[k] == self.cells[k + 2]
            {
                self.winner = Some(self.cells[k]);
                return;
            }
        }

        // check diagonals
        if self.cells[0] != Player::Free
            && self.cells[0] == self.cells[4]
            && self.cells[4] == self.cells[8]
        {
            self.winner = Some(self.cells[4]);
            return;
        }
        if self.cells[2] != Player::Free
            && self.cells[2] == self.cells[4]
            && self.cells[4] == self.cells[6]
        {
            self.winner = Some(self.cells[4]);
        }

        // check draw
        if self.cells.iter().all(|&x| x != Player::Free) {
            self.winner = Some(Player::Draw);
        }
    }

    pub fn get_winner(&self) -> Option<Player> {
        self.winner
    }
}

impl Clone for MiniBoard {
    fn clone(&self) -> MiniBoard {
        MiniBoard {
            cells: self.cells,
            winner: self.winner,
            childs: [0; 27],
            _hash: self._hash,
            _actions: Vec::new(), // actions are not copied to avoid errors. This is computed on the fly
        }
    }
}

impl Debug for MiniBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..3 {
            for j in 0..3 {
                let c = match self.cells[i * 3 + j] {
                    Player::X => 'X',
                    Player::O => 'O',
                    Player::Free => '-',
                    Player::Draw => 'D',
                };
                write!(f, "{} ", c)?;
            }
            writeln!(f)?;
        }
        writeln!(f, "Winner: {:?}", self.winner)?;
        writeln!(f, "Hash: {}", self._hash)?;
        writeln!(f, "Childs: {:?}", self.childs)?;
        Ok(())
    }
}
