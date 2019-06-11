# string_dist
This package contains alot of refactored or re-implemented stirng distance algorithms found in the existing rust ecosystem. I've tried to point out where the original ideas or implementations came from. 

The rational for not doing pull requests to original projects is twofold:

1. my changes were often so pervasive that a PR would probably have been rude
2. I needed (or will need) some common interfaces or inter-functionality between algos and maintaining a meta-package of forked projects that were all but gutted in many places did not make sense.

## Edit distances char-based

* Hamming
* Jaro
* Jaro-Winkler


### todo
* Mong Elken

### Token Based metrics

* NGram
* QGram
* Jaccard
* Cosine

### todo

* TFIDF
* Jensen-Shannon

## Maximum Likelihood Estimation for Ngram Models
### todo
yes, see if the NGram token comparison can fit into an NGram model with log-likelihood and MLE...>


## substring
From teh rust discourse: [substring from a string](https://users.rust-lang.org/t/how-to-get-a-substring-of-a-string/1351/11)

```
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
```
