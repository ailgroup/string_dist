/* TODO
    Do Harley-Seal popcount
    See also [kimwalisch popcount](https://github.com/kimwalisch/primesieve/blob/5062c611402f391f531dd1d081c6969115f7d40c/src/popcount.cpp#L54)
*/

/// popcount with multiply.
/// See also [kimwalisch popcount64](https://github.com/kimwalisch/primesieve/blob/5062c611402f391f531dd1d081c6969115f7d40c/src/popcount.cpp#L21)
#[inline]
pub fn popcount_mult(mut x: u64) -> u64 {
    let m1: u64 = 0x5555555555555555; //binary: 0101...
    let m2: u64 = 0x3333333333333333; //binary: 00110011..
    let m4: u64 = 0x0f0f0f0f0f0f0f0f; //binary:  4 zeros,  4 ones ...
    let h01: u64 = 0x0101010101010101; //the sum of 256 to the power of 0,1,2,3...
    x -= (x >> 1) & m1; //count of each two bits into those 2 bits
    x = (x & m2) + ((x >> 2) & m2); //put count of each 4 bits

    // using u64 means doing (n * H01) >> 24, which returns left 8 bits of n + (n<<8) + ...
    // it will overflow; instead use `wrapping_mul(H01) >> 24`
    ((x + (x >> 4)) & m4).wrapping_mul(h01) >> 24 //put count of each 8 bits; returns 8 bits of n + (n<<8) + ...
}

/// Computes the [Hamming weight](https://en.wikipedia.org/wiki/Hamming_weight) of `x`, the population count, number of bits set to 1.
pub trait HammingWeight {
    fn native(&self) -> u64;
    fn popcount(&self) -> u64;
}
impl HammingWeight for u64 {
    /// native uses rust native `count_ones()`.
    fn native(&self) -> u64 {
        self.count_ones() as u64
    }
    /// popcount uses `popcount_mult()`.
    fn popcount(&self) -> u64 {
        popcount_mult(*self)
    }
}
impl HammingWeight for u32 {
    /// native uses rust native `count_ones()`.
    fn native(&self) -> u64 {
        self.count_ones() as u64
    }
    /// popcount uses `popcount_mult()`.
    fn popcount(&self) -> u64 {
        popcount_mult(*self as u64)
    }
}
impl HammingWeight for &[u8] {
    /// native iterates over vector and folds `count_ones()`.
    fn native(&self) -> u64 {
        self.iter().fold(0, |a, b| a + b.count_ones() as u64)
    }
    /// popcount uses Lauradoux [tree-merging approach](http://web.archive.org/web/20120411185540/http://perso.citi.insa-lyon.fr/claurado/hamming.html)
    /// to compute bitwise [Hamming distance](https://en.wikipedia.org/wiki/Hamming_distance)
    /// between `x` and `y`, number of bits where `x` and `y` differ,
    /// or, number of set bits xor `x` and `y`.
    /// Needs to be benchmarked
    /// Also used [huonw hamming](https://github.com/huonw/hamming/blob/master/src/weight_.rs#L39) for reference.
    fn popcount(&self) -> u64 {
        //tuple for head,buffer,tail to vectorize
        let (head, buffer, tail) = (&self[..1], [[0 as u64; 30]], &self[1..]);
        let count = HammingWeight::native(&head) + HammingWeight::native(&tail);
        lauradoux_for_weight(buffer, count)
    }
}

/// lauradoux_for_weight uses a tree-merge approach... needs to be benchmarked
/// See Lauradoux Cédric's [tree-merging approach](http://web.archive.org/web/20120411185540/http://perso.citi.insa-lyon.fr/claurado/hamming.html)
fn lauradoux_for_weight(buffer: [[u64; 30]; 1], mut count: u64) -> u64 {
    let m1: u64 = 0x5555555555555555; //binary: 0101...
    let m2: u64 = 0x3333333333333333; //binary: 00110011..
    let m4: u64 = 0x0f0f0f0f0f0f0f0f; //binary:  4 zeros,  4 ones ...
    let m8: u64 = 0x00ff00ff00ff00ff; //binary:  8 zeros,  8 ones ...
    for buf in buffer.iter() {
        let mut accum = 0;
        for _j in 0..10 {
            let j = _j * 3;
            let mut c1 = buf[j];
            let mut c2 = buf[j + 1];
            let mut half1 = buf[j + 2];
            let mut half2 = half1;
            half1 &= m1;
            half2 = (half2 >> 1) & m1;
            c1 -= (c1 >> 1) & m1;
            c2 -= (c2 >> 1) & m1;
            c1 += half1;
            c2 += half2;
            c1 = (c1 & m2) + ((c1 >> 2) & m2);
            c1 += (c2 & m2) + ((c2 >> 2) & m2);
            accum += (c1 & m4) + ((c1 >> 4) & m4);
        }
        accum = (accum & m8) + ((accum >> 8) & m8);
        accum = accum + (accum >> 16);
        accum = accum + (accum >> 32);
        count += accum & 0xFFFF;
    }
    count
}

/// DistanceError for handling error when x,y size are not same.
#[derive(Debug, PartialEq)]
pub enum DistanceError {
    Size,
}
/// fmt for DistanceError
impl std::fmt::Display for DistanceError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            DistanceError::Size => write!(f, "ERROR: byte arrays must be the same size"),
        }
    }
}

/// Computes the bitwise [Hamming distance](https://en.wikipedia.org/wiki/Hamming_distance) using rust native `count_ones()` for vectors x,y. If x,y are not the same size returns `DistanceError::Size`.
pub fn distance_native(x: &[u8], y: &[u8]) -> Result<u64, DistanceError> {
    if x.len() != y.len() {
        return Result::Err(DistanceError::Size);
    }
    let d = x
        .iter()
        .zip(y)
        .fold(0, |a, (b, c)| a + (*b ^ *c).count_ones() as u64);
    Ok(d)
}

/// Uses Lauradoux [tree-merging approach](http://web.archive.org/web/20120411185540/http://perso.citi.insa-lyon.fr/claurado/hamming.html)
/// to compute bitwise [Hamming distance](https://en.wikipedia.org/wiki/Hamming_distance)
/// between vectors x,y. If x,y are not the same size returns `DistanceError::Size`.
/// Needs to be benchmarked
/// Also used [huonw hamming](https://github.com/huonw/hamming/blob/master/src/distance_.rs#L65) for reference.
pub fn lauradoux_for_distance(x: &[u8], y: &[u8]) -> Result<u64, DistanceError> {
    if x.len() != y.len() {
        return Result::Err(DistanceError::Size);
    }
    let (head1, buffer1, tail1) = (&x[..1], [[0 as u64; 30]], &x[1..]);
    let (head2, buffer2, tail2) = (&y[..1], [[0 as u64; 30]], &y[1..]);

    let c_head = match distance_native(head1, head2) {
        Ok(v) => v,
        Err(err) => return Result::Err(err),
    };
    let c_tail = match distance_native(tail1, tail2) {
        Ok(v) => v,
        Err(err) => return Result::Err(err),
    };
    let mut count = c_head + c_tail;

    let m1: u64 = 0x5555555555555555; //binary: 0101...
    let m2: u64 = 0x3333333333333333; //binary: 00110011..
    let m4: u64 = 0x0f0f0f0f0f0f0f0f; //binary:  4 zeros,  4 ones ...
    let m8: u64 = 0x00ff00ff00ff00ff; //binary:  8 zeros,  8 ones ...
    for (buf1, buf2) in buffer1.iter().zip(&buffer2) {
        let mut accum = 0;
        for _j in 0..10 {
            let j = _j * 3;
            let mut c1 = buf1[j] ^ buf2[j];
            let mut c2 = buf1[j + 1] ^ buf2[j + 1];
            let mut half1 = buf1[j + 2] ^ buf1[j + 2];
            let mut half2 = half1;
            half1 &= m1;
            half2 = (half2 >> 1) & m1;
            c1 -= (c1 >> 1) & m1;
            c2 -= (c2 >> 1) & m1;
            c1 += half1;
            c2 += half2;
            c1 = (c1 & m2) + ((c1 >> 2) & m2);
            c1 += (c2 & m2) + ((c2 >> 2) & m2);
            accum += (c1 & m4) + ((c1 >> 4) & m4);
        }
        accum = (accum & m8) + ((accum >> 8) & m8);
        accum = accum + (accum >> 16);
        accum = accum + (accum >> 32);
        count += accum & 0xFFFF;
    }
    Ok(count)
}

/// Computes the [Hamming distance](https://en.wikipedia.org/wiki/Hamming_distance) of vectors `x`,`y`, returns `DistanceError::Size` if vectors not same size.
pub trait HammingSpace {
    fn distance(&self, y: &[u8]) -> Result<u64, DistanceError>;
}

impl HammingSpace for &[u8] {
    /// distance computes bitwise [Hamming distance](https://en.wikipedia.org/wiki/Hamming_distance)
    /// for vectors x,y attempting to use the `lauradoux_for_distance()` first and falling back to
    /// `distance_native()` else. Either function will return `DistanceError::Size` if x,y are not
    /// the same size.
    fn distance(&self, y: &[u8]) -> Result<u64, DistanceError> {
        let d = match lauradoux_for_distance(&self, y) {
            Ok(v) => v,
            Err(ref error) if error == &DistanceError::Size => match distance_native(&self, y) {
                Ok(v) => v,
                Err(e) => return Result::Err(e),
            },
            Err(e) => return Result::Err(e),
        };
        Ok(d)
    }
}

#[cfg(test)]
mod tests {
    use quickcheck as qc;
    use rand;
    #[test]
    fn native_weight() {
        let tests = [
            (&[0u8] as &[u8], 0),
            (&[1], 1),
            (&[0xFF], 8),
            (&[0xFF; 10], 8 * 10),
            (&[1; 1000], 1000),
        ];
        for &(v, expected) in &tests {
            assert_eq!(super::HammingWeight::native(&v), expected);
        }
    }
    #[test]
    fn native_popcount_qcheck() {
        fn prop(v: Vec<u8>, misalign: u8) -> qc::TestResult {
            let data = &v[(misalign as usize % 16)..];
            qc::TestResult::from_bool(
                super::HammingWeight::popcount(&data) == super::HammingWeight::native(&data),
            )
        }
        qc::QuickCheck::new()
            .gen(qc::StdGen::new(rand::thread_rng(), 10_000))
            .quickcheck(prop as fn(Vec<u8>, u8) -> qc::TestResult)
    }
    #[test]
    fn weight_huge() {
        let v = vec![0b1001_1101; 10234567];
        //let v = vec![204; 10234567];
        assert_eq!(
            super::HammingWeight::popcount(&&v[..]),
            v[0].count_ones() as u64 * v.len() as u64
        );
        //assert_eq!(51172835 as u64, v[0].count_ones() as u64 * v.len() as u64);
    }
}
