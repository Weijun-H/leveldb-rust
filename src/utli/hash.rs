pub fn hash(data: &[u8], size: usize, seed: u32) -> u32 {
    // Similar to murmur hash
    let m: u32 = 0xc6a4a793;
    let r: u32 = 24;
    let limit: usize = size;
    let mut h: u32 = seed ^ ((size as u32).wrapping_mul(m));

    // Pick up four bytes at a time
    let mut i: usize = 0;
    while i + 4 <= limit {
        let w: u32 = ((data[i + 3] as u32) << 24)
            | ((data[i + 2] as u32) << 16)
            | ((data[i + 1] as u32) << 8)
            | (data[i] as u32);
        i += 4;
        h = h.wrapping_add(w);
        h = h.wrapping_mul(m);
        h ^= h >> 16;
    }

    // Pick up remaining bytes
    match limit - i {
        3 => {
            h = h.wrapping_add((data[i + 2] as u32) << 16);
            h = h.wrapping_add((data[i + 1] as u32) << 8);
            h = h.wrapping_add(data[i] as u32);
            h = h.wrapping_mul(m);
            h ^= h >> r;
        }
        2 => {
            h += (data[i + 1] as u32) << 8;
            h = h.wrapping_add((data[i + 1] as u32) << 8);
            h = h.wrapping_add(data[i] as u32);
            h ^= h >> r;
        }
        1 => {
            h = h.wrapping_add(data[i] as u32);
            h = h.wrapping_mul(m);
            h ^= h >> r;
        }
        _ => {}
    }
    h
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        let data: &[u8] = &[0x00, 0x01, 0x02, 0x03];
        let size: usize = 4;
        let seed: u32 = 0x12345678;
        let expected: u32 = 3934035616;
        let actual: u32 = hash(data, size, seed);
        assert_eq!(expected, actual);
    }
}
