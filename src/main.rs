mod board;

use std::collections::{HashMap, VecDeque};

use board::{Board, MiniBoard, Player};

fn build_graph() -> HashMap<usize, MiniBoard> {
    let mut childs: HashMap<usize, MiniBoard> = HashMap::new();
    let players = [Player::X, Player::O, Player::Draw];

    let root = MiniBoard::new();
    let mut q: VecDeque<MiniBoard> = VecDeque::new();
    q.push_back(root);
    while let Some(mut node) = q.pop_front() {
        let h = node.get_hash();
        if childs.contains_key(&h) {
            continue;
        }

        if !node.is_over() {
            for i in 0..9 {
                for player in players.iter() {
                    if let Some(child) = node.play(*player, i) {
                        q.push_back(child);
                    }
                }
            }
        }

        childs.entry(h).or_insert(node);
    }

    childs
}

fn main() {
    let timer = std::time::Instant::now();

    let g = build_graph();
    eprintln!("Graph: {:?} states", g.len());

    let mut board = Board::new();

    board.play(Player::X, 0, 8, &g);
    board.play(Player::O, 0, 1, &g);
    board.play(Player::X, 0, 2, &g);
    board.play(Player::O, 0, 3, &g);
    board.play(Player::X, 0, 4, &g);
    board.play(Player::O, 0, 5, &g);
    board.play(Player::O, 0, 6, &g);
    board.play(Player::X, 0, 7, &g);
    board.play(Player::O, 0, 0, &g);

    eprintln!("{:?}", board);

    let actions = board.get_possible_actions(&g);
    eprintln!("Actions: {:?}", actions);

    let root = g.get(&18868).unwrap();
    eprintln!("{:?}", root);

    // let root = g.get(&0).unwrap();
    // eprintln!("Root: \n{:?}", root);

    // let root = g.get(&13122).unwrap();
    // eprintln!("Child: \n{:?}", root);

    eprintln!("Time: {:?}", timer.elapsed());
}
