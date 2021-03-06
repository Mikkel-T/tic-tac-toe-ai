use crate::board::{Board, Move, Player};

/// Find the best current move for the current player
pub fn find_best_move(board: Board) -> Move {
    let mut best_score = std::i32::MIN;
    let mut best_move = Move {
        none: true,
        row: 0,
        col: 0,
    };
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
                    best_move = Move {
                        row: i,
                        col: j,
                        none: false,
                    };
                    best_score = score;
                }
            }
        }
    }

    if best_move.none {
        // TODO Error Handling
        panic!("An unexpected error occurred.");
    } else {
        best_move
    }
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
