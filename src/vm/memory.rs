use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use std::ops::{Neg, Sub};

pub struct Memory<T> {
    inner: VecDeque<T>,
    base: usize,
    position: usize,
}

impl<T> Memory<T> {
    pub fn position(&self) -> isize {
        self.position.checked_sub(self.base).map_or_else(
            ||(self.base.sub(self.position) as isize).neg(),
            |r| r as isize
        )
    }

    #[inline]
    pub fn get(&self) -> Option<&T> {
        self.inner.get(self.position)
    }

    #[inline]
    pub fn get_mut(&mut self) -> Option<&mut T> {
        self.inner.get_mut(self.position)
    }
}

impl<T: Default + Clone> Memory<T>
{
    pub fn front(&mut self, n: usize) {
        let remain = n.checked_sub(self.inner.len() - 1 - self.position);
        match remain {
            Some(0) => (),
            Some(1) => {
                self.inner.push_back(Default::default())
            },
            Some(remain) => {
                self.inner.append(&mut VecDeque::from(vec![Default::default(); remain]))
            },
            _ => ()
        }
        self.position += n;
    }

    pub fn next(&mut self) {
        if self.inner.len() - 1 == self.position {
            self.inner.push_back(Default::default())
        }
        self.position += 1;
    }

    pub fn back(&mut self, n: usize) {
        match self.position.checked_sub(n) {
            Some(p) => {
                self.position = p;
            },
            None => {
                let remain = n - self.position;
                let mut inner = VecDeque::from(vec![Default::default(); remain]);
                inner.append(&mut self.inner);
                self.inner = inner;
                self.position = 0;
                self.base += remain;
            }
        }
    }

    pub fn next_back(&mut self) {
        if self.position == 0 {
            self.inner.push_front(Default::default());
            self.base += 1;
        } else {
            self.position -= 1;
        }
    }
}

impl<T: Default> Default for Memory<T> {
    fn default() -> Self {
        Self {
            inner: [Default::default()].into(),
            base: 0,
            position: 0,
        }
    }
}

impl<T: Display> Display for Memory<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut i = 0;
        write!(f, "[")?;
        for c in &self.inner {
            i += 1;
            if i == self.position + 1 {
                write!(f, ">")?;
            }
            write!(f, "{}", c)?;
            if i != self.inner.len() {
                write!(f, ", ")?;
            }
        }
        writeln!(f, "]")
    }
}
