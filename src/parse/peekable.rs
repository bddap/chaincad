// Converts an iterator to one that can be peeked.

use std::mem::swap;

pub struct Peekable <I: Iterator> {
    iter: I,
    n: Option<I::Item>,
}

impl<I: Iterator> Iterator for Peekable<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        let mut sa = None;
        swap(&mut self.n, &mut sa);
        match sa {
            Some(a) => Some(a),
            None => self.iter.next()
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, upper) = self.iter.size_hint();
        let a = match self.n {
            Some(_) => 1,
            None => 0
        };
        (lower + a, upper.map(|u| u + a))
    }

    // fn try_fold<Acc, G, R>(&mut self, init: Acc, mut g: G) -> R where
    //     Self: Sized, G: FnMut(Acc, Self::Item) -> R, R: Try<Ok=Acc>
    // {
    //     let f = &mut self.f;
    //     self.iter.try_fold(init, move |acc, elt| g(acc, f(elt)))
    // }

    // fn fold<Acc, G>(self, init: Acc, mut g: G) -> Acc
    //     where G: FnMut(Acc, Self::Item) -> Acc,
    // {
    //     let mut f = self.f;
    //     self.iter.fold(init, move |acc, elt| g(acc, f(elt)))
    // }
}

impl<I: Iterator> Peekable<I> {
    pub fn peek(mut self) -> Option<I::Item> {
        match self.n {
            None => {
                let mut n = self.next();
                swap(&mut self.n, &mut n);
            },
            _ => {}
        }
        self.n
    }
}
