#![feature(portable_simd)]
#![feature(pointer_is_aligned)]
#![feature(stdsimd)]
#![feature(unchecked_math)]

mod abseil;
use abseil::flat_hash_map::AFHM;
use abseil::flat_hash_set::AFHS;
use abseil::interpretable::*;
use abseil::printable::*;

fn main() {
    let instructions = [
        Instr::new(Op::ADD, 0..20),
        // Instr::new(Op::REMOVE, 4..14),
    ];
    
    let mut amap: AFHM<i32, i32> = AFHM::new();
    // amap.print(true);
    // amap.interpret(&instructions, true);
    for i in 0..20 {
        amap.insert(i, i);
    }

    for i in 0..20 {
        let v1 = amap.get_loc(i);
        let v2 = amap.get_loc_simd(i);
        println!("{} vs {}", v1, v2);
    }
}
