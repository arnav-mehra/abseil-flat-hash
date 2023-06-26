// #![feature(test)]

mod abseil;
use abseil::flat_hash_map::AFHM;

fn main() {
    println!("Hello, world!");

    // let mut ind = 10;
    // for i in 0..10 {
    //     ind += i * 2 + 1;
    //     ind &= 15;
    //     println!("{}", ind);
    // }

    let mut amap: AFHM<i32, i32> = AFHM::new();
    for c in 1..5 {
        amap.insert(c, c);
    }
    for i in 0..amap.capacity() {
        if !amap.is_full(i) { continue; }
        print!("({}: <{}, {}>), ", i, amap.arr[i].0, amap.arr[i].1);
    } println!();
}
