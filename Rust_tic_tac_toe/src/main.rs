use std::io;
use std::process;
use colored::*;

fn initialize_board(size: usize) -> Vec<char> {
    vec![' '; size * size]
}

fn display_board(board: &[char], size: usize) {
    for i in 0..size {
        for j in 0..size {
            let symbol = match board[i * size + j] {
                'X' => "X".green(),
                'O' => "O".blue(),
                _ => " ".normal(),
            };
            print!(" {} ", symbol);
            if j < size - 1 {
                print!("|");
            }
        }
        println!();
        if i < size - 1 {
            for j in 0..size {
                print!("---");
                if j < size - 1 {
                    print!("+");
                }
            }
            println!();
        }
    }
}

fn check_winner(board: &[char], size: usize, player: char) -> bool {
    // Vérifier les lignes et colonnes
    for i in 0..size {
        let row_win = (0..size).all(|j| board[i * size + j] == player);
        let col_win = (0..size).all(|j| board[j * size + i] == player);
        if row_win || col_win {
            return true;
        }
    }

    // Vérifier les diagonales
    let diag1_win = (0..size).all(|i| board[i * size + i] == player);
    let diag2_win = (0..size).all(|i| board[i * size + (size - i - 1)] == player);

    diag1_win || diag2_win
}

fn play_game(size: usize) {
    let mut board = initialize_board(size);
    let mut current_player = 'X';
    let mut moves = 0;
    let max_moves = size * size;

    loop {
        // Clear screen
        print!("\x1B[2J\x1B[1;1H");

        display_board(&board, size);
        println!("Joueur {}, entrez votre coup (ligne et colonne) :", current_player);

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Erreur de lecture");

        let (row, col): (usize, usize) = match input
            .trim()
            .split_whitespace()
            .filter_map(|x| x.parse::<usize>().ok())
            .collect::<Vec<_>>()
            .as_slice()
        {
            [row, col] => (*row, *col),
            _ => {
                continue;
            }
        };

        // coup invalide
        if row >= size || col >= size || board[row * size + col] != ' ' {
            continue;
        }

        board[row * size + col] = current_player;
        moves += 1;

        if check_winner(&board, size, current_player) {
            print!("\x1B[2J\x1B[1;1H"); // clear cli 
            display_board(&board, size);
            println!(
                "Félicitations ! Joueur {} a gagné !",
                match current_player {
                    'X' => current_player.to_string().green(),
                    'O' => current_player.to_string().blue(),
                    _ => current_player.to_string().black(), 
                }
            );
            break;
        }

        if moves == max_moves {
            print!("\x1B[2J\x1B[1;1H"); // clear cli
            display_board(&board, size);
            println!("Match nul !");
            break;
        }

        // Changer de joueur
        current_player = match current_player {
            'X' => 'O',
            'O' => 'X',
            _ => unreachable!(),
        };
    }
}

fn main() {
    println!("Entrez la taille du plateau (ex. 3 pour 3x3) :");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erreur de lecture");

    let size: usize = match input.trim().parse() {
        Ok(n) if n > 0 => n,
        _ => {
            println!("Taille invalide !");
            process::exit(1);
        }
    };

    play_game(size);
}