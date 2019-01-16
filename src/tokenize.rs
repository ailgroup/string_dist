use crate::ratcliff_obershelp as ro;
use std::collections::HashSet;
struct TextSplitter;

/*
    References:

    * [fuzzywuzzy py](https://github.com/seatgeek/fuzzywuzzy)
    * [fuzzywuzzy julia](https://github.com/matthieugomez/StringDistances.jl)
    * [fuzzywuzzy rust](https://github.com/logannc/fuzzyrusty)
*/

/// return tuple with (shortest, longest) string
fn order_by_len<'a>(s1: &'a str, s2: &'a str) -> (&'a str, &'a str) {
    //shortest, longest
    match s1.len() <= s2.len() {
        true => (s1, s2),
        _ => (s2, s1),
    }
}

// RatcliffObserhelp distance
pub fn ratio(s1: &str, s2: &str) -> u8 {
    //total length
    let sumlen = (s1.len() + s2.len()) as f32;
    //find the shorter and longer
    let (short, long) = order_by_len(s1, s2);
    //iter, map, sum last block size
    let similar: usize = ro::matching_blocks(short, long)
        .iter()
        .map(|&(_, _, s)| s)
        .sum();
    if sumlen > 0.0 {
        return (100.0 * (2.0 * (similar as f32) / sumlen)).round() as u8;
    }
    100
}

pub fn partial_ratio(s1: &str, s2: &str) -> u8 {
    let (shorter, longer) = match s1.len() <= s2.len() {
        true => (s1, s2),
        _ => (s2, s1),
    };
    let blocks = ro::matching_blocks(&shorter, &longer);
    let mut max: u8 = 0;
    for (i, _, k) in blocks {
        let substr = &shorter[i..i + k];
        let r = ratio(&shorter, substr);
        if r > 99 {
            return 100;
        } else if r > max {
            max = r;
        }
    }
    max
}

pub fn token_sort_ratio(s1: &str, s2: &str, force_ascii: bool, full_process: bool) -> u8 {
    token_sort(s1, s2, false, force_ascii, full_process)
}

pub fn partial_token_sort_ratio(s1: &str, s2: &str, force_ascii: bool, full_process: bool) -> u8 {
    token_sort(s1, s2, true, force_ascii, full_process)
}

pub fn token_set_ratio(s1: &str, s2: &str, force_ascii: bool, full_process: bool) -> u8 {
    token_set(s1, s2, false, force_ascii, full_process)
}

pub fn partial_token_set_ratio(s1: &str, s2: &str, force_ascii: bool, full_process: bool) -> u8 {
    token_set(s1, s2, true, force_ascii, full_process)
}

pub fn qratio(s1: &str, s2: &str, force_ascii: bool) -> u8 {
    let (p1, p2) = (pre_process(s1, force_ascii), pre_process(s2, force_ascii));
    ratio(&p1, &p2)
}

pub fn uqratio(s1: &str, s2: &str) -> u8 {
    qratio(s1, s2, false)
}

fn process_and_sort(s: &str, force_ascii: bool, process: bool) -> String {
    let ts = if process {
        pre_process(s, force_ascii)
    } else {
        s.to_string()
    };
    let mut ts_split: Vec<_> = ts.split_whitespace().collect();
    ts_split.sort();
    ts_split.join(" ")
}

fn token_sort(s1: &str, s2: &str, partial: bool, force_ascii: bool, process: bool) -> u8 {
    let sorted1 = process_and_sort(s1, force_ascii, process);
    let sorted2 = process_and_sort(s2, force_ascii, process);
    if partial {
        return partial_ratio(sorted1.as_ref(), sorted2.as_ref());
    }
    ratio(sorted1.as_ref(), sorted2.as_ref())
}

fn token_set(s1: &str, s2: &str, partial: bool, force_ascii: bool, process: bool) -> u8 {
    let (p1, p2) = if process {
        (pre_process(s1, force_ascii), pre_process(s2, force_ascii))
    } else {
        (s1.to_string(), s2.to_string())
    };
    let t1: HashSet<_> = p1.split_whitespace().collect();
    let t2: HashSet<_> = p2.split_whitespace().collect();
    let mut intersection: Vec<_> = t1.intersection(&t2).cloned().collect();
    let mut diff1to2: Vec<_> = t1.difference(&t2).cloned().collect();
    let mut diff2to1: Vec<_> = t2.difference(&t1).cloned().collect();
    intersection.sort();
    diff1to2.sort();
    diff2to1.sort();
    let intersect_str = intersection.join(" ");
    let diff1to2_str = diff1to2.join(" ");
    let diff2to1_str = diff2to1.join(" ");
    let combined_1to2 = if diff1to2_str.len() > 0 {
        intersect_str.to_string() + &diff1to2_str
    } else {
        intersect_str.to_string()
    };
    let combined_2to1 = if diff2to1_str.len() > 0 {
        intersect_str.to_string() + &diff2to1_str
    } else {
        intersect_str.to_string()
    };
    if partial {
        vec![
            partial_ratio(&intersect_str, &combined_1to2),
            partial_ratio(&intersect_str, &combined_2to1),
            partial_ratio(&combined_1to2, &combined_2to1),
        ]
        .iter()
        .max()
        .unwrap()
        .clone()
    } else {
        vec![
            ratio(&intersect_str, &combined_1to2),
            ratio(&intersect_str, &combined_2to1),
            ratio(&combined_1to2, &combined_2to1),
        ]
        .iter()
        .max()
        .unwrap()
        .clone()
    }
}

fn pre_process(s: &str, force_ascii: bool) -> String {
    let mut result = s.to_string();
    if force_ascii {
        result = result.chars().filter(|s| s.is_ascii()).collect();
    }
    result = result
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { ' ' })
        .collect();
    result.make_ascii_lowercase();
    result.trim().to_string()
}

impl TextSplitter {
    fn word_match(c: char) -> bool {
        match c {
            ' ' | ',' | '.' | '!' | '?' | ';' | '\'' | '"' | ':' | '\t' | '\n' | '(' | ')'
            | '-' => true,
            _ => false,
        }
    }
    fn word_to_tokens_match(c: char) -> bool {
        match c {
            ' ' | ',' | '.' | '(' | ')' | '-' => true,
            _ => false,
        }
    }
}
/// words takes borrowed str and splits on word_match
/// filtering out empty slots
pub fn words(t: &str) -> Vec<&str> {
    t.split(TextSplitter::word_match)
        .filter(|s| !s.is_empty())
        .collect()
}

/// tokens takes borrowed str and splits all
/// filtering out empty slots
pub fn tokens(t: &str) -> Vec<&str> {
    t.split("").filter(|s| !s.is_empty()).collect()
}

/// split_tokens_lower takes borrowed str
/// and uses custom splitter, collects, concats,
/// then lowercase each str, returning new String
pub fn tokens_lower(t: &str) -> String {
    t.split(TextSplitter::word_to_tokens_match)
        .collect::<Vec<&str>>()
        .concat()
        .split("")
        .map(|s| s.to_lowercase())
        .filter(|s| !s.is_empty()) //needed a split pads
        .collect::<Vec<String>>()
        .concat()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn identity() {
        assert_eq!(ratio("hello", "hello"), 100);
    }

    #[test]
    fn world() {
        assert_eq!(ratio("hello test", "hello world"), 57);
        assert_eq!(ratio("hello test", "hello worlasdfasd"), 52);
    }

    #[test]
    fn case_insensitive() {
        assert!(ratio("hello WORLD", "hello world") != 100);
        assert_eq!(
            ratio(
                &pre_process("hello WORLD", false),
                &pre_process("hello world", false)
            ),
            100
        );
    }

    #[test]
    fn token_sort() {
        assert_eq!(
            token_sort_ratio("hello world", "world hello", false, false),
            100
        );
    }

    #[test]
    fn partial() {
        assert_eq!(partial_ratio("hello", "hello world"), 100);
    }

    #[test]
    fn partial_token_sort() {
        assert_eq!(
            partial_token_set_ratio(
                "new york mets vs atlanta braves",
                "atlanta braves vs new york mets",
                false,
                false
            ),
            100
        );
    }

    #[test]
    fn token_set() {
        assert_eq!(
            token_set_ratio(
                "new york mets vs atlanta braves",
                "atlanta braves vs new york mets",
                false,
                false
            ),
            100
        );
    }

    #[test]
    fn partial_token_set() {
        assert_eq!(
            partial_token_set_ratio(
                "new york mets vs atlanta braves",
                "new york city mets - atlanta braves",
                false,
                false
            ),
            100
        );
    }
}
