// src/game.rs
use crate::board::Board;
use crate::piece::{Color, Piece};
use stylus_sdk::{prelude::*, Address};


pub struct Game {
    board: Board,
    turn: Color,
    player1: Address,
    player2: Address,
    moves: Vec<(usize, usize, usize, usize)>, // Store moves as (from_x, from_y, to_x, to_y)
    game_over: bool,
    winner: Option<Color>,
}

impl Game {
    pub fn new(player1: Address, player2: Address) -> Self {
        Game {
            board: Board::new(),
            turn: Color::White,
            player1,
            player2,
            moves: vec![],
            game_over: false,
            winner: None,
        }
    }


        pub fn play_turn(&mut self, from: (usize, usize), to: (usize, usize), caller: Address) -> Result<(), String> {
            if self.game_over {
                return Err("The game is over.".into());
            }
    
            if caller != self.current_player() {
                return Err("It's not your turn.".into());
            }
    
            // Validate and perform the move
            self.board.move_piece(from, to)?;
    
            // Record the move
            self.moves.push((from.0, from.1, to.0, to.1));
    
            // Switch turns
            self.turn = match self.turn {
                Color::White => Color::Black,
                Color::Black => Color::White,
            };
    
            Ok(())
        }
    
        pub fn is_game_over(&self) -> bool {
            self.game_over
        }
    
        pub fn declare_winner(&mut self, winner: Color) {
            self.game_over = true;
            self.winner = Some(winner);
        }


    fn current_player(&self) -> Address {
        match self.turn {
            Color::White => self.player1,
            Color::Black => self.player2,
        }
    }
}