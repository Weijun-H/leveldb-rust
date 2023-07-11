use std::{cmp, ops::Index};

#[derive(Debug)]
pub struct Slice {
    pub data: Box<[u8]>,
    pub size: usize,
}

impl Slice {
    pub fn new(data: Box<[u8]>, size: usize) -> Self {
        Slice { data, size }
    }

    pub fn start_with(&self, prefix: &Slice) -> bool {
        if self.size < prefix.size {
            return false;
        }
        cmp::PartialEq::eq(&self.data[..prefix.size], &prefix.data[..prefix.size])
    }

    pub fn remove_prefix(&mut self, n: usize) {
        if n >= self.size {
            self.size = 0;
        } else {
            self.size -= n;
            self.data = self.data[n..].into();
        }
    }

    pub fn empty(&self) -> bool {
        self.size == 0
    }

    pub fn clear(&mut self) {
        self.size = 0;
        self.data = vec![].into();
    }
}

impl Index<usize> for Slice {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl PartialEq for Slice {
    fn eq(&self, other: &Self) -> bool {
        if self.size != other.size {
            return false;
        }
        cmp::PartialEq::eq(&self.data[..self.size], &other.data[..other.size])
    }
}

impl PartialOrd for Slice {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(cmp::PartialOrd::partial_cmp(
            &self.data[..self.size],
            &other.data[..other.size],
        )?)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_slice() {
        let data: Vec<u8> = vec![0x00, 0x01, 0x02, 0x03];
        let size: usize = 4;
        let slice: Slice = Slice::new(data.into(), size);
        assert_eq!(slice[0], 0x00);
        assert_eq!(slice[1], 0x01);
        assert_eq!(slice[2], 0x02);
        assert_eq!(slice[3], 0x03);
    }

    #[test]
    fn test_slice_eq() {
        let data: Vec<u8> = vec![0x00, 0x01, 0x02, 0x03];
        let size: usize = 4;
        let slice1: Slice = Slice::new(data.clone().into(), size);
        let slice2: Slice = Slice::new(data.clone().into(), size);
        assert_eq!(slice1, slice2);
        assert!(slice1 == slice2);
    }

    #[test]
    fn test_slice_partial_cmp() {
        let data1: Vec<u8> = vec![0x00, 0x01, 0x02, 0x03];
        let size1: usize = 4;
        let data2: Vec<u8> = vec![0x00, 0x01, 0x02, 0x04];
        let size2: usize = 4;
        let slice1: Slice = Slice::new(data1.into(), size1);
        let slice2: Slice = Slice::new(data2.into(), size2);
        assert_eq!(slice1.partial_cmp(&slice2), Some(cmp::Ordering::Less));
        assert!(slice1 < slice2);
    }
}
