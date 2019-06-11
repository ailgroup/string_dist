/*
RatcliffObserhelp distance

SEE tokenizer

 [NIST Ratcliff/Obershelp pattern recognition](https://xlinux.nist.gov/dads/HTML/ratcliffObershelp.html):
Compute the similarity of two strings as the number of matching characters divided by the total number of characters in the two strings.
Matching characters are those in the longest common subsequence plus, recursively, matching characters in the unmatched region on either side of the longest common subsequence.
*/

/*
// use this to replace longest_common_substring
// Return start of commn substring in s1, start of common substring in s2, and length of substring
// Indexes refer to character number, not index (differ for Unicode strings)
fn longest_common_substring() -> (usize, usize, usize) {
    //https://github.com/matthieugomez/StringDistances.jl/blob/2834265e96f18993a98a57d97e9f27a450a161d1/src/distances/RatcliffObershelp.jl#L3
    (0, 0, 0)
}
*/

fn longest_common_substring<'a>(
    shorter: &'a str,
    longer: &'a str,
    low1: usize,
    high1: usize,
    low2: usize,
    high2: usize,
) -> (usize, usize, usize) {
    let longsub = &longer[low2..high2];
    let slen = high1 - low1;
    for size in (1..=slen + 1).rev() {
        for start in 0..=slen - size {
            let substr = &shorter[low1 + start..low1 + start + size];
            let matches: Vec<(usize, &'a str)> = longsub.match_indices(substr).collect();
            // Does this need to be sorted?
            if let Some(&(startb, matchstr)) = matches.first() {
                return (low1 + start, low2 + startb, matchstr.len());
            }
        }
    }
    (low1, low2, 0)
}

/*
// use this to replace matching_blocks
fn matching_blocks() -> Vec<(usize, usize, usize)> {
    //https://github.com/matthieugomez/StringDistances.jl/blob/2834265e96f18993a98a57d97e9f27a450a161d1/src/distances/RatcliffObershelp.jl#L31
    vec![(0, 0, 0)]
}
*/

pub fn matching_blocks<'a>(shorter: &'a str, longer: &'a str) -> Vec<(usize, usize, usize)> {
    let (len1, len2) = (shorter.len(), longer.len());
    let mut queue: Vec<(usize, usize, usize, usize)> = vec![(0, len1, 0, len2)];
    let mut matching_blocks = Vec::new();
    while let Some((low1, high1, low2, high2)) = queue.pop() {
        let (i, j, k) = longest_common_substring(shorter, longer, low1, high1, low2, high2);
        if k != 0 {
            matching_blocks.push((i, j, k));
            if low1 < i && low2 < j {
                queue.push((low1, i, low2, j));
            }
            if i + k < high1 && j + k < high2 {
                queue.push((i + k, high1, j + k, high2));
            }
        }
    }
    //matching_blocks.sort(); // Is this necessary?
    let (mut i1, mut j1, mut k1) = (0, 0, 0);
    let mut non_adjacent = Vec::new();
    for (i2, j2, k2) in matching_blocks {
        if i1 + k1 == i2 && j1 + k1 == j2 {
            k1 += k2;
        } else {
            if k1 != 0 {
                non_adjacent.push((i1, j1, k1));
            }
            i1 = i2;
            j1 = j2;
            k1 = k2;
        }
    }
    if k1 != 0 {
        non_adjacent.push((i1, j1, k1));
    }
    non_adjacent.push((len1, len2, 0));
    non_adjacent
}





/*
## substring
From the rust discourse: [substring from a string](https://users.rust-lang.org/t/how-to-get-a-substring-of-a-string/1351/11)

This looks like an interesting take....

use std::ops::{Bound, RangeBounds};

trait StringUtils {
    fn substring(&self, start: usize, len: usize) -> &str;
    fn slice(&self, range: impl RangeBounds<usize>) -> &str;
}

impl StringUtils for str {
    fn substring(&self, start: usize, len: usize) -> &str {
        let mut char_pos = 0;
        let mut byte_start = 0;
        let mut it = self.chars();
        loop {
            if char_pos == start { break; }
            if let Some(c) = it.next() {
                char_pos += 1;
                byte_start += c.len_utf8();
            }
            else { break; }
        }
        char_pos = 0;
        let mut byte_end = byte_start;
        loop {
            if char_pos == len { break; }
            if let Some(c) = it.next() {
                char_pos += 1;
                byte_end += c.len_utf8();
            }
            else { break; }
        }
        &self[byte_start..byte_end]
    }
    fn slice(&self, range: impl RangeBounds<usize>) -> &str {
        let start = match range.start_bound() {
            Bound::Included(bound) | Bound::Excluded(bound) => *bound,
            Bound::Unbounded => 0,
        };
        let len = match range.end_bound() {
            Bound::Included(bound) => *bound + 1,
            Bound::Excluded(bound) => *bound,
            Bound::Unbounded => self.len(),
        } - start;
        self.substring(start, len)
    }
}

fn main() {
    let s = "abcdèfghij";
    // All three statements should print:
    // "abcdè, abcdèfghij, dèfgh, dèfghij."
    println!("{}, {}, {}, {}.",
        s.substring(0, 5),
        s.substring(0, 50),
        s.substring(3, 5),
        s.substring(3, 50));
    println!("{}, {}, {}, {}.",
        s.slice(..5),
        s.slice(..50),
        s.slice(3..8),
        s.slice(3..));
    println!("{}, {}, {}, {}.",
        s.slice(..=4),
        s.slice(..=49),
        s.slice(3..=7),
        s.slice(3..));
}
*/