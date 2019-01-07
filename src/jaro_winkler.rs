use std::cmp::{max, min};

/// jaro_winkler_distance: higher score is less similar.
/// computes 6 parameters, 3 of which are calculated internally.
/// s1_len,s2_len are the length of input string
/// p is the prefix scale, ranges between [0,0.25], gives more favorable ratings to strings that match from the beginning for a set prefix length l.
/// m is matching chars, the number of shared symbols
/// t number of transpositions
/// l is the length of common prefix, the number of symbols at beginning before first mismatch (max is 4)
///
///     References:
///
///     * [jaro-winkler wikipedia](https://en.wikipedia.org/wiki/Jaro%E2%80%93Winkler_distance)
///
/// jwd = 1 - (sim_j * (1 - l*p))
/// this is more performant than the often defined jwd = 1 - sim_jwr due to less calculations in the jaro winkler similarity function.
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
    (1.0 - jaro_similarity(s1, s2)) * (1.0 - l * p)
}

/// jaro_winkler_similarity: higher score is more similar.
/// sim_jw = 1 - jwd : jwd calculates 1-sim_jaro * (1-l*p)
///
/// often: sim_jw = sim_j + l*p(1-sim_j);
/// a more performant calculation: 1 - jwd : jwd calculates 1-sim_jaro * (1-l*p)
pub fn jaro_winkler_similarity(s1: &str, s2: &str, p: f64) -> f64 {
    1.0 - jaro_winkler_distance(s1, s2, p)
}

// jaro_func(A,B.m,t,l,p) = f(A,B,m,t) * (1 - l*p)
/// a is length of first string
/// b is length of second string
/// m is matching chars, the number of shared symbols
/// t is number of needed transpositions of shared symbols
fn calculate(a: f64, b: f64, m: f64, t: f64) -> f64 {
    (1.0 / 3.0) * (m / a + m / b + (m - t) / m)
}

/// jaro_similarity: higher score is more similar.
///     References:
///
///     * [jaro-winkler wikipedia](https://en.wikipedia.org/wiki/Jaro%E2%80%93Winkler_distance)
///
/// m is matching chars, the number of shared symbols
/// t is number of needed transpositions fo shared symbols
/// l is the length of common prefix, the number of symbols at beginning before first mismatch (max is 4)
fn jaro_similarity(s1: &str, s2: &str) -> f64 {
    //exact strings no need to calculate
    if s1 == s2 {
        return 1.0;
    }
    let s1_char_count = s1.chars().count();
    let s2_char_count = s2.chars().count();
    //zero char count, "empty" string are equidistant
    if s1_char_count == 0 || s2_char_count == 0 {
        return 1.0;
    }
    // since s1 == s2 already been checked, we know these are extreme
    if s1_char_count == 1 && s2_char_count == 1 {
        return 0.0;
    }
    // m matching chars, t transpositions
    let (mut m, mut t) = (0.0, 0.0);
    // s1, s2 matching if they are the same and not farther than window
    let window = (max(s1_char_count, s2_char_count) / 2) - 1;
    // index we can increment tandem with window
    let mut s2_index: usize = 0;
    // fill vec with 0==false for postional checks
    let mut match_idx: Vec<u8> = Vec::with_capacity(s2_char_count as usize);
    for _i in 0..s2_char_count {
        match_idx.push(0)
    }

    // TODO refactor to iterator for next
    for (i, a) in s1.chars().enumerate() {
        //define proper min,max for window for s2 index comparisons
        let min_limit = if i > window { max(0, i - window) } else { 0 };
        let max_limit = min(s2_char_count - 1, i + window);
        // next
        if min_limit > max_limit {
            continue;
        }

        for (j, b) in s2.chars().enumerate() {
            //check for match
            if min_limit <= j && j <= max_limit && a == b && match_idx[j] == 0 {
                // found match, set 1==true, increment m
                match_idx[j] = 1;
                m += 1.0;
                // check transpose,
                if j < s2_index {
                    t += 1.0;
                }
                // increment index, move on
                s2_index = j;
                break;
            }
        }
    }

    if m == 0.0 {
        return 0.0;
    }
    calculate(s1_char_count as f64, s2_char_count as f64, m, t)
}
