use super::flat_hash_set::AFHS;

pub trait Printable {
    fn print(&self, print_all : bool);
}

impl Printable for AFHS<i32> {
    fn print(&self, print_all : bool) {
        for i in 0..self.capacity() {
            if self.is_full(i) {
                print!("{} ", self.arr[i]);
            } else if print_all {
                print!("X ");
            }
        }
        println!();
    }
}