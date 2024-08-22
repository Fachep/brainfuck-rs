use std::io::Read;
use crate::node::{BlockNode, Node};
use crate::vm::memory::Memory;

pub trait Execute<T> {
    fn execute(&self, memory: &mut Memory<T>);
}

macro_rules! execute_impl {
    ($( $t:ty ) *) => {
        $(
        impl Execute<$t> for Node {
            fn execute(&self, memory: &mut Memory<$t>) {
                let mut input = std::io::stdin().bytes();
                match self {
                    Node::Root(inner) => {
                        for n in inner {
                            n.execute(memory);
                        }
                    }
                    Node::Block(block) => {
                        while *memory.get().unwrap() != 0 {
                            for n in &block.inner {
                                n.execute(memory);
                            }
                        }
                    }
                    Node::Increase => {
                        *memory.get_mut().unwrap() += 1;
                    }
                    Node::Decrease => {
                        *memory.get_mut().unwrap() -= 1;
                    }
                    Node::Front => {
                        memory.next();
                    }
                    Node::Back => {
                        memory.next_back();
                    }
                    Node::Input => {
                        *memory.get_mut().unwrap() = input.next().unwrap().unwrap() as $t;
                    }
                    Node::Output => {
                        print!("{}", *memory.get().unwrap() as u8 as char);
                    }
                }
            }
        }
        )*

    };
}

execute_impl!(i8 u8 i16 u16);
