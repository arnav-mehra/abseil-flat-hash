#![feature(portable_simd)]

mod abseil;
use abseil::flat_hash_map::AFHM;
use abseil::flat_hash_set::AFHS;
use abseil::interpretable::*;
use abseil::printable::*;

fn main() {
    let instructions = [
        Instr::new(Op::ADD, 0..20),
        Instr::new(Op::REMOVE, 4..14),
    ];
    
    let mut amap: AFHS<i32> = AFHS::new();
    amap.print(true);
    amap.interpret(&instructions, true);
}
