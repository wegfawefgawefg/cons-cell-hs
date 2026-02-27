#[derive(Clone)]
enum List<T> {
    Empty,
    Cons(T, Box<List<T>>),
}

use List::{Cons, Empty};

impl<T> List<T> {
    fn cons(head: T, tail: List<T>) -> List<T> {
        Cons(head, Box::new(tail))
    }

    // Convert into a Rust iterator for laziness
    fn into_iter(self) -> impl Iterator<Item = T> {
        std::iter::successors(Some(self), |acc| match acc {
            Some(Cons(_, xs)) => Some(*xs), // This line needs correction
            _ => None,
        })
        .filter_map(|list| match list {
            Cons(x, _) => Some(x),
            Empty => None,
        })
    }
}

impl<T: std::fmt::Display> std::fmt::Display for List<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        let mut list = self;
        while let Cons(x, xs) = list {
            write!(f, "{}", x)?;
            list = xs;
            if let Cons(_, _) = list {
                write!(f, ", ")?;
            }
        }
        write!(f, "]")
    }
}

fn ttake<T: Copy>(n: usize, list: List<T>) -> List<T> {
    List::into_iter(list)
        .take(n)
        .fold(Empty, |acc, x| List::cons(x, acc))
}

fn cons_range(start: i32, end: i32) -> List<i32> {
    (start..=end).fold(Empty, |acc, x| List::cons(x, acc))
}

fn double_up<T: Copy>(list: List<T>) -> List<T> {
    List::into_iter(list)
        .flat_map(|x| std::iter::repeat(x).take(2))
        .fold(Empty, |acc, x| List::cons(x, acc))
}

fn main() {
    let one_and_two = cons_range(1, 2);
    let two_to_the_million = std::iter::repeat_with(|| one_and_two.clone())
        .take(10)
        .fold(one_and_two, |acc, _| double_up(acc));
    let first_ten = ttake(10, two_to_the_million);
    println!("{}", first_ten);
}
