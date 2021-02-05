mod command;
mod draw;
mod error;
mod field;

use bitvec::prelude::*;
use command::Move;
use error::BishopArtError;
use field::Field;
use rand::{thread_rng, Rng};

fn main() -> Result<(), BishopArtError> {
    let mut field = Field::new();
    let mut rng = thread_rng();
    let hash: [u8; 16] = rng.gen();
    // let hash = [
    //     0xfcu8, 0x94, 0xb0, 0xc1, 0xe5, 0xb0, 0x98, 0x7c, 0x58, 0x43, 0x99, 0x76, 0x97, 0xee, 0x9f,
    //     0xb7,
    // ];
    let bit_slice = BitSlice::<Msb0, _>::from_slice(&hash).unwrap();

    // let digest = md5::compute(b"abcdefghijklmnopqrstuvwxyz");
    // let bit_slice = BitSlice::<Msb0, _>::from_slice(digest.as_ref()).unwrap();
    let mut moves = Vec::new();
    for pair in bit_slice.chunks_exact(2) {
        let bishop_move = match pair {
            p if p == bits![0, 0] => Move::NorthWest,
            p if p == bits![0, 1] => Move::NorthEast,
            p if p == bits![1, 0] => Move::SouthWest,
            p if p == bits![1, 1] => Move::SouthEast,
            _ => unreachable!(),
        };

        moves.push(bishop_move)
    }

    moves
        .into_iter()
        .for_each(|direction| field.make_move(direction));
    field.finalize();

    field.draw();

    Ok(())
}
