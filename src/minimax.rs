use crate::board::{Board, Move, Player};
use rand::seq::SliceRandom;
use rand::thread_rng;

/// Find the best current move for the current player
pub fn find_best_move(board: Board) -> Move {
    let mut best_score = std::i32::MIN;
    let mut best_moves: Vec<Move> = Vec::new();
    for (i, row) in board.rows.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if cell.is_none() {
                let mut tmp_board = board;
                tmp_board
                    .turn(Move {
                        row: i,
                        col: j,
                        none: false,
                    })
                    .unwrap();
                let score = minimax(
                    tmp_board,
                    false,
                    board.turn,
                    0,
                    std::i32::MIN,
                    std::i32::MAX,
                );

                if score > best_score {
                    best_moves = vec![Move {
                        row: i,
                        col: j,
                        none: false,
                    }];

                    best_score = score;
                } else if score == best_score {
                    best_moves.push(Move {
                        row: i,
                        col: j,
                        none: false,
                    });
                }
            }
        }
    }

    let mut rng = thread_rng();
    let best_move = best_moves.choose(&mut rng);
    if best_move.is_none() {
        // TODO Error Handling
        panic!("An unexpected error occurred.");
    }

    best_move.unwrap().to_owned()
}

/// Minimax algorithm to find the best move for the current player
fn minimax(
    board: Board,
    is_maximzing: bool,
    player: Player,
    depth: i32,
    mut alpha: i32,
    mut beta: i32,
) -> i32 {
    let result = board.check_winner();
    if result.is_some() {
        return result.unwrap().score(player);
    }

    if is_maximzing {
        let mut best_score = std::i32::MIN;
        for (i, row) in board.rows.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if cell.is_none() {
                    let mut tmp_board = board;
                    tmp_board
                        .turn(Move {
                            row: i,
                            col: j,
                            none: false,
                        })
                        .unwrap();
                    let score = minimax(tmp_board, false, player, depth + 1, alpha, beta);
                    best_score = std::cmp::max(score - depth, best_score);
                    alpha = std::cmp::max(alpha, best_score);
                    if beta <= alpha {
                        break;
                    }
                }
                if beta <= alpha {
                    break;
                }
            }
        }
        best_score
    } else {
        let mut best_score = std::i32::MAX;
        for (i, row) in board.rows.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if cell.is_none() {
                    let mut tmp_board = board;
                    tmp_board
                        .turn(Move {
                            row: i,
                            col: j,
                            none: false,
                        })
                        .unwrap();
                    let score = minimax(tmp_board, true, player, depth + 1, alpha, beta);
                    best_score = std::cmp::min(score + depth, best_score);
                    beta = std::cmp::min(beta, best_score);
                    if beta <= alpha {
                        break;
                    }
                }
                if beta <= alpha {
                    break;
                }
            }
        }
        best_score
    }
}
