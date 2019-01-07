use std::collections::HashSet;
struct TextSplitter;

/*
    References:

    * [fuzzywuzzy py](https://github.com/seatgeek/fuzzywuzzy)
    * [fuzzywuzzy julia](https://github.com/matthieugomez/StringDistances.jl)
    * [fuzzywuzzy rust](https://github.com/logannc/fuzzyrusty)
*/

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

// RatcliffObserhelp distance
// https://github.com/matthieugomez/StringDistances.jl/blob/2834265e96f18993a98a57d97e9f27a450a161d1/src/distances/RatcliffObershelp.jl#L54
pub fn ratio(s1: &str, s2: &str) -> u8 {
    let (shorter, longer) = if s1.len() <= s2.len() {
        (s1, s2)
    } else {
        (s2, s1)
    };
    let matches: usize = get_matching_blocks(shorter, longer)
        .iter()
        .map(|&(_, _, s)| s)
        .sum();
    let sumlength: f32 = (s1.len() + s2.len()) as f32;
    if sumlength > 0.0 {
        (100.0 * (2.0 * (matches as f32) / sumlength)).round() as u8
    } else {
        100
    }
}

pub fn partial_ratio(s1: &str, s2: &str) -> u8 {
    let (shorter, longer) = if s1.len() <= s2.len() {
        (s1.to_string(), s2.to_string())
    } else {
        (s2.to_string(), s1.to_string())
    };
    let blocks = get_matching_blocks(&shorter, &longer);
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
        partial_ratio(sorted1.as_ref(), sorted2.as_ref())
    } else {
        ratio(sorted1.as_ref(), sorted2.as_ref())
    }
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

/*
// use this to replace find_longest_match
// Return start of commn substring in s1, start of common substring in s2, and length of substring
// Indexes refer to character number, not index (differ for Unicode strings)
fn longest_common_substring() -> (usize, usize, usize) {
    //https://github.com/matthieugomez/StringDistances.jl/blob/2834265e96f18993a98a57d97e9f27a450a161d1/src/distances/RatcliffObershelp.jl#L3
    (0, 0, 0)
}
*/

fn find_longest_match<'a>(
    shorter: &'a str,
    longer: &'a str,
    low1: usize,
    high1: usize,
    low2: usize,
    high2: usize,
) -> (usize, usize, usize) {
    let longsub = &longer[low2..high2];
    let slen = high1 - low1;
    for size in (1..slen + 1).rev() {
        for start in 0..slen - size + 1 {
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
// use this to replace get_matching_blocks
fn matching_blocks() -> Vec<(usize, usize, usize)> {
    //https://github.com/matthieugomez/StringDistances.jl/blob/2834265e96f18993a98a57d97e9f27a450a161d1/src/distances/RatcliffObershelp.jl#L31
    vec![(0, 0, 0)]
}
*/

fn get_matching_blocks<'a>(shorter: &'a str, longer: &'a str) -> Vec<(usize, usize, usize)> {
    let (len1, len2) = (shorter.len(), longer.len());
    let mut queue: Vec<(usize, usize, usize, usize)> = vec![(0, len1, 0, len2)];
    let mut matching_blocks = Vec::new();
    while let Some((low1, high1, low2, high2)) = queue.pop() {
        let (i, j, k) = find_longest_match(shorter, longer, low1, high1, low2, high2);
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
    matching_blocks.sort(); // Is this necessary?
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
