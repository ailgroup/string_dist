use std::cmp::{max, min};
// jaro_func(A,B.m,t,l,p) = f(A,B,m,t) * (1 - l*p)
/// a is length of first string
/// b is length of second string
/// m is matching chars, the number of shared symbols
/// t is number of needed transpositions fo shared symbols
/// l is the length of common prefix, the number of symbols at beginning before first mismatch (max is 4)
/// p is the prefix scale, ranges between [0,0.25], gives more favorable ratings to strings that match from the beginning for a set prefix length l.
fn jaro_func(a: f64, b: f64, m: f64, t: f64) -> f64 {
    (1.0 / 3.0) * (m / a + m / b + (m - t) / m)
}

/// jaro_distance: higher score is less similar.
/// m is matching chars, the number of shared symbols
/// t is number of needed transpositions fo shared symbols
/// l is the length of common prefix, the number of symbols at beginning before first mismatch (max is 4)
/// p is the prefix scale, ranges between [0,0.25], gives more favorable ratings to strings that match from the beginning for a set prefix length l.
pub fn jaro(s1: &str, s2: &str) -> f64 {
    if s1 == s2 {
        return 1.0;
    }
    let s1_char_count = s1.chars().count();
    let s2_char_count = s2.chars().count();
    if s1_char_count == 0 || s2_char_count == 0 || (s1_char_count == 1 && s2_char_count == 1) {
        return 0.0;
    }
    let (mut t, mut m) = (0.0, 0.0);
    let mut s2_index: usize = 0;
    let mut sa2: Vec<u8> = Vec::with_capacity(s2_char_count as usize);

    for _i in 0..s2_char_count {
        sa2.push(0)
    }
    let window = (max(s1_char_count, s2_char_count) / 2) - 1;
    for (i, a) in s1.chars().enumerate() {
        let min_limit = if i > window { max(0, i - window) } else { 0 };
        let max_limit = min(s2_char_count - 1, i + window);
        if min_limit > max_limit {
            continue;
        }

        for (j, b) in s2.chars().enumerate() {
            if min_limit <= j && j <= max_limit && a == b && sa2[j] == 0 {
                sa2[j] = 1;
                m += 1.0;
                if j < s2_index {
                    t += 1.0;
                }
                s2_index = j;
                break;
            }
        }
    }

    if m == 0.0 {
        return 0.0;
    }
    jaro_func(s1_char_count as f64, s2_char_count as f64, m, t)
}

pub fn jaro_winkler_distance(s1: &str, s2: &str, mut p: f64) -> f64 {
    let mut l = s1
        .chars()
        .zip(s2.chars())
        .take_while(|&(s1_char, s2_char)| s1_char == s2_char)
        .count() as f64;
    if l > 4.0 {
        l = 4.0;
    };
    if p > 0.25 {
        p = 0.25
    }

    (1.0 - jaro(s1, s2)) * (1.0 - l * p)
}

pub fn jaro_winkler_similarity(s1: &str, s2: &str, p: f64) -> f64 {
    1.0 - jaro_winkler_distance(s1, s2, p)
}
