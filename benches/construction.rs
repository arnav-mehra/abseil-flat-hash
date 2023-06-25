// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::vec;
// use crate::abseil::flat_hash_map::AFHM;
// use crate::abseil::flat_hash_set::AFHS;

// extern crate test;
// use test::Bencher;

// #[bench]
// pub fn build_std(b: &mut Bencher) {
//     b.iter(|| {
//         let mut smap: HashMap<i32, i32> = HashMap::new();
//         for c in 1..8 {
//             smap.insert(c, c);
//         }
//     });
// }

// #[bench]
// pub fn build_abs(b: &mut Bencher) {
//     b.iter(|| {
//         let mut amap: AFHM<i32, i32> = AFHM::new();
//         for c in 1..5 {
//             amap.insert(c, c);
//         }
//         for i in amap.arr.iter() {
//             print!("({}, {}), ", i.0, i.1);
//         } println!();
//     });
// }