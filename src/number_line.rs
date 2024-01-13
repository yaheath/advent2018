use std::cmp::{max, PartialEq};
use std::ops::{Bound, Index, IndexMut, RangeBounds};
use std::slice::Iter;
use std::vec::Vec;

#[derive(Clone)]
pub struct NumberLine<T: Copy + PartialEq> {
    min_idx: i64,
    max_idx: i64,
    data: Vec<T>,
    default_val: T,
}
impl<T: Copy + PartialEq> NumberLine<T> {
    pub fn new(min_idx: i64, max_idx: i64, default_val: T) -> Self {
        assert!(max_idx >= min_idx);
        Self {
            min_idx,
            max_idx,
            data: vec![default_val; (max_idx - min_idx + 1) as usize],
            default_val,
        }
    }
    pub fn from_initial(initial: &[T], default_val: T) -> Self {
        let mut data: Vec<T> = vec![default_val; initial.len()];
        data.copy_from_slice(initial);
        Self {
            min_idx: 0,
            max_idx: data.len() as i64 - 1,
            data,
            default_val,
        }
    }
    pub fn len(&self) -> usize { self.data.len() }
    pub fn is_empty(&self) -> bool { self.data.is_empty() }
    pub fn start_index(&self) -> i64 { self.min_idx }
    pub fn end_index(&self) -> i64 { self.max_idx + 1 }
    /*
    pub fn clone(&self) -> Self {
        let mut data: Vec<T> = vec![self.default_val; self.data.len()];
        data.copy_from_slice(&self.data[..]);
        Self {
            min_idx: self.min_idx,
            max_idx: self.max_idx,
            data,
            default_val: self.default_val,
        }
    }
    */
    pub fn iter(&self) -> Iter<T> { self.data.iter() }
    pub fn enumerate(&self) -> NumberLineEnumerator<T> {
        NumberLineEnumerator::new(self)
    }
}
impl<T: Copy + PartialEq> Index<i64> for NumberLine<T> {
    type Output = T;
    fn index(&self, idx: i64) -> &Self::Output {
        if idx < self.min_idx || idx > self.max_idx {
            &self.default_val
        }
        else {
            &self.data[(idx - self.min_idx) as usize]
        }
    }
}
impl<T: Copy + PartialEq> IndexMut<i64> for NumberLine<T> {
    fn index_mut(&mut self, idx: i64) -> &mut Self::Output {
        let datalen = self.data.len() as i64;
        if idx < self.min_idx {
            let ext = max(self.min_idx - idx, 32);
            let newlen = datalen + ext;
            self.data.resize(newlen as usize, self.default_val);
            self.data.copy_within(0 .. (newlen - ext) as usize, ext as usize);
            for i in 0 .. ext as usize {
                self.data[i] = self.default_val
            }
            self.min_idx -= ext;
        }
        else if idx - self.min_idx >= datalen {
            let ext = max(idx - self.min_idx - datalen + 1, 32);
            let newlen = datalen + ext;
            self.data.resize(newlen as usize, self.default_val);
            self.max_idx += ext;
        }
        &mut self.data[(idx - self.min_idx) as usize]
    }
}
impl<T: Copy + PartialEq> RangeBounds<i64> for NumberLine<T> {
    fn start_bound(&self) -> Bound<&i64> { Bound::Included(&self.min_idx) }
    fn end_bound(&self) -> Bound<&i64> { Bound::Included(&self.max_idx) }
}

pub struct NumberLineEnumerator<'a, T: Copy + PartialEq> {
    obj: &'a NumberLine<T>,
    idx: i64,
    max_idx: i64,
}
impl<'a, T: Copy + PartialEq> NumberLineEnumerator<'a, T> {
    pub fn new(obj: &'a NumberLine<T>) -> Self {
        let mut min_idx = obj.min_idx;
        let mut max_idx = obj.min_idx - 1;
        if let Some(end) = obj.iter().rev().enumerate()
                              .find(|x| *(x.1) != obj.default_val) {
            max_idx = obj.min_idx + (obj.data.len() - end.0 - 1) as i64;
            while obj[min_idx] == obj.default_val {
                min_idx += 1;
            }
        }
        Self {
            obj,
            idx: min_idx,
            max_idx,
        }
    }
}
impl<T: Copy + PartialEq> Iterator for NumberLineEnumerator<'_, T> {
    type Item = (i64, T);
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx > self.max_idx {
            None
        }
        else {
            let r = (self.idx, self.obj[self.idx]);
            self.idx += 1;
            Some(r)
        }
    }
}

#[cfg(test)]
mod test {
    use super::NumberLine;

    #[test]
    fn numberline() {
        let mut foo: NumberLine<bool> = NumberLine::new(1, 10, false);
        assert!(!foo[0]);
        assert!(!foo[1]);
        assert!(!foo[10]);
        assert!(!foo[333]);
        assert!(!foo[-333]);
        assert_eq!(foo.len(), 10);

        foo[2] = true;
        assert!(foo[2]);
        assert_eq!(foo.len(), 10);
        foo[4] = true;

        let mut e = foo.enumerate();
        assert_eq!(e.next(), Some((2, true)));
        assert_eq!(e.next(), Some((3, false)));
        assert_eq!(e.next(), Some((4, true)));
        assert_eq!(e.next(), None);

        foo[-2] = true;
        assert!(foo[-2]);
        assert!(!foo[-1]);
        assert!(!foo[0]);
        assert!(!foo[1]);
        assert!(foo[2]);
        assert!(!foo[3]);
        assert_eq!(foo.len(), 42);

        foo[20] = true;
        assert!(foo[2]);
        assert!(!foo[19]);
        assert!(foo[20]);
        assert!(!foo[21]);
        assert_eq!(foo.len(), 74);
    }
}
