use brainfuck_rs::jit::runtime::Runtime;
use brainfuck_rs::node::Node;
use brainfuck_rs::vm::execute::Execute;
use brainfuck_rs::vm::memory::Memory;

fn main() {
    let s = ",[.[-],]";
    let mut r = Node::try_parse(s.chars()).unwrap();
    let mut rt = Runtime::default();
    r.compile(&mut rt);
    println!("{:?}", rt.codes());
    rt.run();
}
