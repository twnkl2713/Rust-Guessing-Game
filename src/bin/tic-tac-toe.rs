use std::io;

const PLAYER_X: char = 'X';
const PLAYER_O: char = 'O';
const BOARD_SIZE: usize = 3;

type Board = [[char; BOARD_SIZE]; BOARD_SIZE];

fn initialize_board() -> Board {
    [[' '; BOARD_SIZE]; BOARD_SIZE]
}

fn render_cell(cell: char) -> &'static str {
    match cell {
        'X' => "❌",
        'O' => "⭕",
        _ => "◻️",
    }
}

fn print_board(board: &Board) {
    println!("\n   0   1   2");
    for (i, row) in board.iter().enumerate() {
        print!("{} ", i);
        for (j, cell) in row.iter().enumerate() {
            print!(" {} ", render_cell(*cell));
            if j < BOARD_SIZE - 1 {
                print!("|");
            }
        }
        println!();
        if i < BOARD_SIZE - 1 {
            println!("  ---+---+---");
        }
    }
}

fn get_player_move(current_player: char, board: &Board) -> (usize, usize) {
    loop {
        println!("🎮 Player {} (enter row and column):", render_cell(current_player));
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        let coordinates: Vec<usize> = input
            .split_whitespace()
            .filter_map(|s| s.parse::<usize>().ok())
            .collect();

        if coordinates.len() == 2 {
            let (row, col) = (coordinates[0], coordinates[1]);
            if row < BOARD_SIZE && col < BOARD_SIZE && board[row][col] == ' ' {
                return (row, col);
            }
        }
        println!("❌ Invalid move! Try again.");
    }
}

fn check_winner(player: char, board: &Board) -> bool {
    for i in 0..BOARD_SIZE {
        if (board[i][0] == player && board[i][1] == player && board[i][2] == player) || (board[0][i] == player && board[1][i] == player && board[2][i] == player) {
            return true;
        }
    }

    (board[0][0] == player && board[1][1] == player && board[2][2] == player)
        || (board[0][2] == player && board[1][1] == player && board[2][0] == player)
}

fn check_draw(board: &Board) -> bool {
    board.iter().all(|row| row.iter().all(|&cell| cell != ' '))
}

fn prompt_restart() -> bool {
    println!("\n🔁 Do you want to play again? (y/n):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().eq_ignore_ascii_case("y")
}

fn play_game() {
    let mut board = initialize_board();
    let mut current_player = PLAYER_X;

    loop {
        print_board(&board);
        let (row, col) = get_player_move(current_player, &board);
        board[row][col] = current_player;

        if check_winner(current_player, &board) {
            print_board(&board);
            println!("\n🎉 Player {} wins!", render_cell(current_player));
            break;
        }

        if check_draw(&board) {
            print_board(&board);
            println!("\n🤝 It's a draw!");
            break;
        }

        current_player = if current_player == PLAYER_X { PLAYER_O } else { PLAYER_X };
    }
}

fn main() {
    println!("🎮 Welcome to Emoji Tic Tac Toe 🧠");

    loop {
        play_game();
        if !prompt_restart() {
            println!("👋 Thanks for playing!");
            break;
        }
    }
}
