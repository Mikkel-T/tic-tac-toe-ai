#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
pub mod board;
mod minimax;

use board::{Board, GameResult, Move, Player};
use minimax::find_best_move;

use druid::widget::{Button, Flex, Label, Painter, SizedBox};
use druid::{
    theme, AppLauncher, Color, Data, Env, Lens, RenderContext, Widget, WidgetExt, WindowDesc,
};

#[derive(Clone, Data, Lens)]
struct TicTacToeState {
    player: Option<Player>,
    message: String,
    board: Board,
}

fn main() {
    let main_window = WindowDesc::new(build_root_widget())
        .title("Tic Tac Toe")
        .window_size((400.0, 470.0));

    // Initialize state
    let initial_state = TicTacToeState {
        player: None,
        board: Board::new(),
        message: String::from("Start the game, or pick a player"),
    };

    // Launch app
    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn cell(row: usize, col: usize) -> impl Widget<TicTacToeState> {
    let painter = Painter::new(move |ctx, data: &TicTacToeState, env| {
        let bounds = ctx.size().to_rect();

        ctx.fill(bounds, &env.get(theme::BACKGROUND_LIGHT));

        if data.board.rows[row][col].is_none() {
            if ctx.is_hot() {
                ctx.stroke(bounds.inset(-0.5), &Color::WHITE, 1.0);
            }

            if ctx.is_active() {
                ctx.fill(bounds, &Color::rgb8(0x71, 0x71, 0x71));
            }
        }
    });

    Label::new(move |data: &TicTacToeState, _env: &Env| {
        format!("{}", board::cell_to_string(data.board.rows[row][col]))
    })
    .with_text_size(50.)
    .center()
    .background(painter)
    .expand()
    .on_click(move |_ctx, data: &mut TicTacToeState, _env| {
        if data.player.is_none() {
            data.player = Some(data.board.turn);
        }

        if data.board.result.is_none() && data.board.rows[row][col].is_none() {
            data.board
                .turn(Move {
                    row: row,
                    col: col,
                    none: false,
                })
                .unwrap();
            if data.board.result.is_none() {
                data.board.turn(find_best_move(data.board)).unwrap();
            }
        }
    })
}

fn row(row: usize) -> impl Widget<TicTacToeState> {
    Flex::row()
        .with_flex_child(cell(row, 0), 1.0)
        .with_spacer(1.0)
        .with_flex_child(cell(row, 1), 1.0)
        .with_spacer(1.0)
        .with_flex_child(cell(row, 2), 1.0)
}

fn pick_player() -> impl Widget<TicTacToeState> {
    let painter_x = Painter::new(move |ctx, data: &TicTacToeState, env| {
        let bounds = ctx.size().to_rect();

        ctx.fill(bounds, &env.get(theme::BACKGROUND_LIGHT));

        if ctx.is_hot() {
            ctx.stroke(bounds.inset(-0.5), &Color::WHITE, 1.0);
        }
        if data.player.is_none() || (data.player.is_some() && data.player.unwrap() == Player::X) {
            ctx.fill(bounds, &Color::rgb8(0x71, 0x71, 0x71));
        }
    });

    let painter_o = Painter::new(move |ctx, data: &TicTacToeState, env| {
        let bounds = ctx.size().to_rect();

        ctx.fill(bounds, &env.get(theme::BACKGROUND_LIGHT));

        if ctx.is_hot() {
            ctx.stroke(bounds.inset(-0.5), &Color::WHITE, 1.0);
        }
        if data.player.is_some() && data.player.unwrap() == Player::O {
            ctx.fill(bounds, &Color::rgb8(0x71, 0x71, 0x71));
        }
    });

    Flex::row()
        .with_child(
            SizedBox::new(Label::new("X").center().background(painter_x).on_click(
                move |_ctx, data: &mut TicTacToeState, _env| {
                    data.board = Board::new();
                    data.player = Some(Player::X);
                    data.message = String::new();
                },
            ))
            .width(50.)
            .height(50.),
        )
        .with_spacer(30.)
        .with_child(
            SizedBox::new(Label::new("O").center().background(painter_o).on_click(
                move |_ctx, data: &mut TicTacToeState, _env| {
                    data.board = Board::new();
                    data.player = Some(Player::O);
                    data.message = String::new();
                    data.board.turn(find_best_move(data.board)).unwrap();
                },
            ))
            .width(50.)
            .height(50.),
        )
}

fn build_root_widget() -> impl Widget<TicTacToeState> {
    let message = Label::new(move |data: &TicTacToeState, _env: &Env| format!("{}", data.message));

    let board = Flex::column()
        .with_flex_child(row(0), 1.0)
        .with_spacer(1.0)
        .with_flex_child(row(1), 1.0)
        .with_spacer(1.0)
        .with_flex_child(row(2), 1.0)
        .must_fill_main_axis(true);

    let winner_label =
        Label::new(
            move |data: &TicTacToeState, _env: &Env| match data.board.result {
                Some(result) => match result {
                    GameResult::P(_) => format!("{} wins!", result),
                    GameResult::Tie => format!("{}", result),
                },
                None => "".to_string(),
            },
        );

    let restart_btn = Button::new("Restart").on_click(|_ctx, data: &mut TicTacToeState, _env| {
        data.board = Board::new();
        match data.player {
            Some(player) => match player {
                Player::X => (),
                Player::O => data.board.turn(find_best_move(data.board)).unwrap(),
            },
            None => (),
        }
    });

    // Collect all the widgets
    Flex::column()
        .with_spacer(10.)
        .with_child(message)
        .with_spacer(10.)
        .with_flex_child(pick_player(), 1.0)
        .with_spacer(10.)
        .with_flex_child(board, 5.0)
        .with_spacer(10.)
        .with_child(winner_label)
        .with_spacer(10.)
        .with_child(restart_btn)
        .with_spacer(10.)
}
