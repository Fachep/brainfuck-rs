use std::mem;
use std::ops::{AddAssign, SubAssign};

#[derive(Debug)]
pub enum Error {
    BlockEnd,
    IteratorEnd,
    UnknownChar(char),
    Other(String),
}

type Result<R> = std::result::Result<R, Error>;

trait Parse: Sized {
    fn parse(iter: &mut impl Iterator<Item=char>) -> Result<Self>;
}

#[derive(Debug)]
pub enum Node {
    Root(Vec<Node>),
    Block(BlockNode),
    Increase,
    Decrease,
    Front,
    Back,
    Input,
    Output,
}

impl Node {
    pub fn try_parse(iter: impl IntoIterator<Item=char>) -> Result<Self> {
        let mut iter = iter.into_iter();
        let mut inner = Vec::with_capacity(iter.size_hint().0);
        while let res = Node::parse(&mut iter) {
            match res {
                Ok(n) => inner.push(n),
                Err(Error::IteratorEnd) => break,
                Err(Error::UnknownChar(_)) => (),
                _ => return res,
            }
        };
        inner.shrink_to_fit();
        Ok(Self::Root(inner))
    }
}

impl Parse for Node
{
    fn parse(iter: &mut impl Iterator<Item=char>) -> Result<Self> {
        match iter.next() {
            None => Err(Error::IteratorEnd),
            Some(c) => match c {
                '+' => Ok(Node::Increase),
                '-' => Ok(Node::Decrease),
                '>' => Ok(Node::Front),
                '<' => Ok(Node::Back),
                '[' => BlockNode::parse(iter).map(Node::Block),
                ']' => Err(Error::BlockEnd),
                '.' => Ok(Node::Output),
                ',' => Ok(Node::Input),
                _ => Err(Error::UnknownChar(c)),
            }
        }
    }
}

#[derive(Debug)]
pub struct BlockNode {
    pub(crate) inner: Vec<Node>
}

impl Parse for BlockNode
{
    fn parse(iter: &mut impl Iterator<Item=char>) -> Result<Self> {
        let mut inner = Vec::new();
        while let res = Node::parse(iter) {
            match res {
                Ok(n) => inner.push(n),
                Err(Error::BlockEnd) => break,
                Err(Error::UnknownChar(_)) => (),
                Err(e) => return Err(e),
            }
        };
        Ok(Self {
            inner
        })
    }
}
