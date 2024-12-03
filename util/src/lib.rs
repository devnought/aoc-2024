use arraydeque::ArrayDeque;

pub struct IterWindow<T, const N: usize>
where
    T: Iterator,
{
    iter: T,
    buffer: ArrayDeque<T::Item, N>,
}

impl<T, const N: usize> Iterator for IterWindow<T, N>
where
    T: Iterator,
    [<T as Iterator>::Item; N]: for<'a> TryFrom<&'a [<T as Iterator>::Item]>,
{
    type Item = [T::Item; N];

    fn next(&mut self) -> Option<Self::Item> {
        // First run, full buffer
        if self.buffer.is_empty() {
            for _ in 0..N {
                self.buffer.push_back(self.iter.next()?).ok()?;
            }
        } else {
            self.buffer.pop_front();
            self.buffer.push_back(self.iter.next()?).ok()?;
        }

        self.buffer.linearize();
        let (back, _) = self.buffer.as_slices();

        back.try_into().ok()
    }
}

pub trait IterWindowIterator<T>: Iterator<Item = T> + Sized {
    fn iter_window<const N: usize>(self) -> IterWindow<Self, N> {
        IterWindow {
            iter: self,
            buffer: ArrayDeque::new(),
        }
    }
}

impl<T, I> IterWindowIterator<T> for I where I: Iterator<Item = T> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let data = [0u8, 2, 4, 6, 8];
        let data_pairs = data.into_iter().iter_window::<2>().collect::<Vec<_>>();

        println!("{data_pairs:?}");

        assert_eq!(4, data_pairs.len());
        assert_eq!(vec![[0, 2], [2, 4], [4, 6], [6, 8]], data_pairs);
    }
}
