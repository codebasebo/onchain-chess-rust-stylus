// src/board.rs
use crate::piece::{Piece, Color};

#[derive(Debug, Clone)]
pub struct Board {
    grid: [[Option<Piece>; 8]; 8],
}

impl Board {
    impl Board {
        pub fn new() -> Self {
            let mut grid = [[None; 8]; 8];
    
            // Place pawns
            for i in 0..8 {
                grid[1][i] = Some(Piece { color: Color::White, piece_type: PieceType::Pawn });
                grid[6][i] = Some(Piece { color: Color::Black, piece_type: PieceType::Pawn });
            }
    
            // Place rooks
            grid[0][0] = Some(Piece { color: Color::White, piece_type: PieceType::Rook });
            grid[0][7] = Some(Piece { color: Color::White, piece_type: PieceType::Rook });
            grid[7][0] = Some(Piece { color: Color::Black, piece_type: PieceType::Rook });
            grid[7][7] = Some(Piece { color: Color::Black, piece_type: PieceType::Rook });
    
            // Place knights
            grid[0][1] = Some(Piece { color: Color::White, piece_type: PieceType::Knight });
            grid[0][6] = Some(Piece { color: Color::White, piece_type: PieceType::Knight });
            grid[7][1] = Some(Piece { color: Color::Black, piece_type: PieceType::Knight });
            grid[7][6] = Some(Piece { color: Color::Black, piece_type: PieceType::Knight });
    
            // Place bishops
            grid[0][2] = Some(Piece { color: Color::White, piece_type: PieceType::Bishop });
            grid[0][5] = Some(Piece { color: Color::White, piece_type: PieceType::Bishop });
            grid[7][2] = Some(Piece { color: Color::Black, piece_type: PieceType::Bishop });
            grid[7][5] = Some(Piece { color: Color::Black, piece_type: PieceType::Bishop });
    
            // Place queens
            grid[0][3] = Some(Piece { color: Color::White, piece_type: PieceType::Queen });
            grid[7][3] = Some(Piece { color: Color::Black, piece_type: PieceType::Queen });
    
            // Place kings
            grid[0][4] = Some(Piece { color: Color::White, piece_type: PieceType::King });
            grid[7][4] = Some(Piece { color: Color::Black, piece_type: PieceType::King });
    
            Board { grid }
        }
    }

    pub fn get_piece(&self, x: usize, y: usize) -> Option<&Piece> {
        self.grid[x][y].as_ref()
    }


        pub fn move_piece(&mut self, from: (usize, usize), to: (usize, usize)) -> Result<(), String> {
            let (from_x, from_y) = from;
            let (to_x, to_y) = to;
    
            // Ensure coordinates are within bounds
            if from_x >= 8 || from_y >= 8 || to_x >= 8 || to_y >= 8 {
                return Err("Coordinates out of bounds.".into());
            }
    
            // Ensure there's a piece at the source square
            let piece = match self.grid[from_x][from_y].as_ref() {
                Some(p) => p,
                None => return Err("No piece at source square.".into()),
            };
    
            // Ensure the destination square is empty or contains an opponent's piece
            if let Some(dest_piece) = &self.grid[to_x][to_y] {
                if dest_piece.color == piece.color {
                    return Err("Cannot capture your own piece.".into());
                }
            }
    
            // Perform the move
            self.grid[to_x][to_y] = self.grid[from_x][from_y].take();
    
            Ok(())
        }
    
}