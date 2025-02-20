// src/piece.rs
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Color {
    White,
    Black,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Piece {
    pub color: Color,
    pub piece_type: PieceType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}