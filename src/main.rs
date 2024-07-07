mod board;

use std::collections::{HashMap, VecDeque};

use board::{MiniBoard, Player};

fn build_graph() -> HashMap<usize, MiniBoard> {
    let mut childs: HashMap<usize, MiniBoard> = HashMap::new();
    let mut visited: [bool; 19683] = [false; 19683];
    let players = [Player::X, Player::O];

    let root = MiniBoard::new();
    let mut q: VecDeque<MiniBoard> = VecDeque::new();
    q.push_back(root);
    while let Some(mut node) = q.pop_front() {
        let h = node.get_hash();
        if visited[h] {
            continue;
        }
        visited[h] = true;

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
    eprintln!("Graph: {:?}", g.len());

    let root = g.get(&0).unwrap();
    eprintln!("Root: \n{:?}", root);

    let root = g.get(&13122).unwrap();
    eprintln!("Child: \n{:?}", root);

    eprintln!("Time: {:?}", timer.elapsed());
}
