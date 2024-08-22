use crate::jit::runtime::Runtime;
use crate::node::Node;

impl Node {
    pub fn compile(&self, rt: &mut Runtime) {
        let Node::Root(inner) = self else { return; };
        let mut codes = Self::compile_nodes(inner, rt);
        codes.push(0xC3);
        rt.set_codes(codes);
    }

    fn compile_node(&self, rt: &Runtime) -> Vec<u8> {
        match self {
            Node::Root(inner) => Self::compile_nodes(inner, rt),
            Node::Block(block) => {
                let mut codes = vec![0x80, 0x39, 0x00, 0x0F, 0x84, 0x00, 0x00, 0x00, 0x00];
                codes.append(&mut Self::compile_nodes(&block.inner, rt));
                codes.extend([0x80, 0x39, 0x00]);
                let len = codes.len();
                if len + 2 - 10 < 0x80 {
                    codes.extend([0x75, 0xFD - len as u8 + 10]);
                } else if len + 6 - 10 < 0x8000_0000 {
                    codes.extend([0x0F, 0x85]);
                    codes.extend((0xFFFF_FFF9 - len as u32 + 10).to_ne_bytes());
                } else {
                    unimplemented!()
                }
                let jmp = (codes.len() as u32 - 9).to_ne_bytes();
                codes[5..9].copy_from_slice(&jmp);
                codes
            },
            Node::Increase => vec![0xfe, 0x01],
            Node::Decrease => vec![0xfe, 0x09],
            Node::Front => vec![0x48, 0xff, 0xc1],
            Node::Back => vec![0x48, 0xff, 0xc9],
            Node::Input => {
                let mut codes = vec![0x48, 0xB8];
                codes.extend(rt.rt_input().to_ne_bytes());
                codes.extend([
                    0xFF, 0xD0,
                    0x48, 0x89, 0xC1,
                ]);
                codes
            }
            Node::Output => {
                let mut codes = vec![0x48, 0xB8];
                codes.extend(rt.rt_output().to_ne_bytes());
                codes.extend([
                    0xFF, 0xD0,
                    0x48, 0x89, 0xC1,
                ]);
                codes
            }
            _ => unreachable!()
        }
    }

    fn compile_nodes(nodes: &Vec<Node>, rt: &Runtime) -> Vec<u8> {
        let mut codes = Vec::new();
        let mut iter = nodes.iter().peekable();
        while let Some(n) = iter.next() {
            codes.append(&mut n.compile_node(rt));
            let m = iter.peek();
            match (n, m) {
                (Node::Front | Node::Back, Some(Node::Decrease | Node::Increase
                                                | Node::Input | Node::Output
                                                | Node::Block(_)) | None)
                     => match n {
                        Node::Front => {
                            codes.extend([
                                // 0x48, 0x39, 0xCA,
                                // 0x76, 0x0F,
                                0x48, 0xB8,
                            ]);
                            codes.extend(rt.rt_mem_extend().to_ne_bytes());
                            codes.extend([
                                0xFF, 0xD0,
                                0x48, 0x89, 0xC1,
                            ]);
                        }
                        Node::Back => {
                            codes.extend([
                                // 0x48, 0x39, 0xCB,
                                // 0x7E, 0x0F,
                                0x48, 0xB8,
                            ]);
                            codes.extend(rt.rt_mem_extend().to_ne_bytes());
                            codes.extend([
                                0xFF, 0xD0,
                                0x48, 0x89, 0xC1,
                            ]);
                        }
                        _ => unreachable!()
                    }
                _ => ()
            }
        }
        codes
    }
}
