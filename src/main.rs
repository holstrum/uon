use rand::Rng;
use std::collections::VecDeque;

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Empty,
    X,
    O,
}

const BOARD_SIZE: usize = 3;

struct Game {
    board: [[Cell; BOARD_SIZE]; BOARD_SIZE],
    x_moves: VecDeque<(usize, usize)>,
    o_moves: VecDeque<(usize, usize)>,
}

impl Game {
    fn new() -> Self {
        Self {
            board: [[Cell::Empty; BOARD_SIZE]; BOARD_SIZE],
            x_moves: VecDeque::new(),
            o_moves: VecDeque::new(),
        }
    }

    fn print_board(&self) {
        for row in &self.board {
            for cell in row {
                print!(
                    "{} ",
                    match cell {
                        Cell::Empty => ".",
                        Cell::X => "X",
                        Cell::O => "O",
                    }
                );
            }
            println!();
        }
    }

    fn place_mark(&mut self, cell_type: Cell) {
        let mut rng = rand::thread_rng();
        loop {
            let row = rng.gen_range(0..BOARD_SIZE);
            let col = rng.gen_range(0..BOARD_SIZE);

            if self.board[row][col] == Cell::Empty {
                self.board[row][col] = cell_type;

                let moves = match cell_type {
                    Cell::X => &mut self.x_moves,
                    Cell::O => &mut self.o_moves,
                    _ => unreachable!(),
                };

                moves.push_back((row, col));
                if moves.len() > 3 {
                    let (old_row, old_col) = moves.pop_front().unwrap();
                    self.board[old_row][old_col] = Cell::Empty;
                }

                break;
            }
        }
    }

    fn play_turn(&mut self) {
        self.place_mark(Cell::X);
        self.place_mark(Cell::O);
    }
}

fn main() {
    let mut game = Game::new();

    for round in 0..20 {
        println!("\nRound {}", round + 1);
        game.play_turn();
        game.print_board();
    }
}