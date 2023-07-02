use super::printable::{*, self};
use super::flat_hash_map::AFHM;
use super::flat_hash_set::AFHS;

#[derive(Clone)]
pub enum Op {
    ADD,
    REMOVE,
}

pub struct Instr {
    pub op : Op,
    pub nums : Vec<i32>
}

impl Instr {
    pub fn new(op: Op, iter : impl Iterator<Item = i32>) -> Instr {
        Instr {
            op,
            nums: iter.collect()
        }
        
    }
}

pub trait Interpretable {
    fn interpret(&mut self, instructions : &[Instr], print_steps : bool) {
        for instr in instructions {
            match instr.op {
                Op::ADD => {
                    for &i in instr.nums.iter() {
                        self.add_op(i);
                    }
                },
                Op::REMOVE => {
                    for &i in instr.nums.iter() {
                        self.remove_op(i);
                    }
                }
            }
            if print_steps {
                self.print_step();
            }
        }
    }

    fn print_step(&self);
    fn add_op(&mut self, x : i32);
    fn remove_op(&mut self, x : i32);
}

impl Interpretable for AFHS<i32> {
    fn add_op(&mut self, x : i32) {
        self.add(x);
    }

    fn remove_op(&mut self, x : i32) {
        self.remove(x);
    }

    fn print_step(&self) {
        self.print(true);
    }
}