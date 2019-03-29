use array_tool::vec::{Intersect, Union, Uniq};
use std::borrow::Cow;
use std::cmp::Ordering;
/*


    References:

    * [fuzzywuzzy py](https://github.com/seatgeek/fuzzywuzzy)
    * [fuzzywuzzy julia](https://github.com/matthieugomez/StringDistances.jl)
    * [fuzzywuzzy rust](https://github.com/logannc/fuzzyrusty)
*/

////////////////////////////////////////////////////////////////////////////////////////
//////// primary functions

/*
token_sort evaluates similarity of the longest substrings (Ratcliff/Obershelp) based on sorted order of the tokens.
pass in two strings, the sorter, and the similarity:

   token_sort(s1, s2, &TokenCmp::new_sort, &TokenCmp::partial_similarity)
   token_sort(s1, s2, &TokenCmp::new_sort_join, &TokenCmp::similarity)

'new_sort' is by default concat (no whitespaces in evaled strings); 'new_sort_join' will be by " ".
*/
pub fn token_sort<'a>(
    t1: &'a str,
    t2: &'a str,
    sorter: &Fn(
        std::vec::Vec<std::borrow::Cow<'a, str>>,
        std::vec::Vec<std::borrow::Cow<'a, str>>,
    ) -> TokenCmp<'a>,
    rat: &Fn(&TokenCmp<'a>) -> u8,
) -> u8 {
    let an = AlphaNumericTokenizer;
    rat(&sorter(an.sequencer(t1), an.sequencer(t2)))
}

/*
token_set evaluates similarity of the longest substrings (Ratcliff/Obershelp) based on the set of intersection and union of tokens.
pass in two strings and the similarity:

   token_set(s1, s2, &TokenCmp::partial_similarity)
   token_set(s1, s2, &TokenCmp::similarity)

'new_sort' is by default concat (no whitespaces in evaled strings); 'new_sort_join' will be by " ".
*/
pub fn token_set<'a>(s1: &'a str, s2: &'a str, rat: &Fn(&TokenCmp<'a>) -> u8) -> u8 {
    let an = AlphaNumericTokenizer;
    let (p1, p2) = (an.sequencer(s1), an.sequencer(s2));
    let mut s1_i_s2 = p1.intersect(p2.clone());
    let mut s1q = p1.uniq(p2.clone()); //diff1to2
    let mut s2q = p2.uniq(p1.clone()); //diff2to1

    s1_i_s2.sort();
    s1q.sort();
    s2q.sort();

    let s1_i_s2_u_s1q = s1_i_s2.union(s1q.clone()); //combined_1to2
    let s1_i_s2_u_s2q = s1_i_s2.union(s2q.clone()); //combined_2to1

    vec![
        rat(&TokenCmp::new_set(s1_i_s2.clone(), s1_i_s2_u_s1q.clone())),
        rat(&TokenCmp::new_set(s1_i_s2, s1_i_s2_u_s2q.clone())),
        rat(&TokenCmp::new_set(s1_i_s2_u_s1q, s1_i_s2_u_s2q)),
    ]
    .iter()
    .cloned()
    .u8_max()
}

////////////////////////////////////////////////////////////////////////////////////////
//////// traits
pub trait CharFilter {
    fn is_char(&self, c: char) -> bool;
}
pub trait TokenizerPos<'a> {
    /// A Tokenizer always needs to produce an Iterator of Tokens.
    type TokenIter: Iterator<Item = TokenPositional<'a>>;
    /// Takes the input string and tokenizes it based on the implementations rules.
    fn tokenize_pos(&self, input: &'a str) -> Self::TokenIter;
    fn pos_summary(&self, tokens: Vec<TokenPositional>) -> TokenizerPosSummary;
}
pub trait TokenizerSeq<'a> {
    type TokenIter: Iterator<Item = TokenSequence<'a>>;
    /// Takes the input string and tokenizes it based on the implementations rules.
    fn tokenize_seq(&self, input: &'a str) -> Self::TokenIter;
    fn sequencer(&self, input: &'a str) -> Vec<std::borrow::Cow<'a, str>>;
    fn token(&self, input: &'a str) -> String;
}
// U8IterExt Iterator Extension for min/max u8
trait U8IterExt {
    fn u8_min(&mut self) -> u8;
    fn u8_max(&mut self) -> u8;
}

//////// structs
pub struct AlphaNumericCharFilter;
pub struct AlphaNumericTokenizer;
pub struct CharTokenPosIter<'a, F: CharFilter> {
    filter: F,
    input: &'a str,
    byte_offset: usize,
    char_offset: usize,
    position: usize,
}
pub struct CharTokenSeqIter<'a, F: CharFilter> {
    filter: F,
    input: &'a str,
    byte_offset: usize,
    char_offset: usize,
    position: usize,
}
pub struct TokenCmp<'a> {
    term1: Cow<'a, str>,
    term2: Cow<'a, str>,
}
pub struct TokenizerNaive;
pub struct TokenizerPosSummary {
    pub sequence: String,
    pub seqlen: usize,
    pub token_strings: Vec<String>,
    pub offsets: Vec<usize>,
    pub positions: Vec<usize>,
}
pub struct TokenPositional<'a> {
    term: Cow<'a, str>,
    start_offset: usize,
    position: usize,
}
pub struct TokenSequence<'a> {
    term: Cow<'a, str>,
}
pub struct WhiteSpaceCharFilter;
pub struct WhiteSpaceTokenizer;

////////////////////////////////////////////////////////////////////////////////////////
//////// impls
impl CharFilter for AlphaNumericCharFilter {
    fn is_char(&self, c: char) -> bool {
        !c.is_alphanumeric()
    }
}
impl<'a, F: CharFilter> CharTokenPosIter<'a, F> {
    pub fn new(filter: F, input: &'a str) -> Self {
        CharTokenPosIter {
            filter: filter,
            input: input,
            byte_offset: 0,
            char_offset: 0,
            position: 0,
        }
    }
}
impl<'a, F: CharFilter> CharTokenSeqIter<'a, F> {
    pub fn new(filter: F, input: &'a str) -> Self {
        CharTokenSeqIter {
            filter: filter,
            input: input,
            byte_offset: 0,
            char_offset: 0,
            position: 0,
        }
    }
}
impl<'a, F: CharFilter> Iterator for CharTokenPosIter<'a, F> {
    type Item = TokenPositional<'a>;
    fn next(&mut self) -> Option<TokenPositional<'a>> {
        let mut skipped_bytes = 0;
        let mut skipped_chars = 0;

        //start from byte offset
        self.input[self.byte_offset..]
            .char_indices()
            .enumerate()
            .skip_while(|&(_, (_, c))| {
                if (self.filter).is_char(c) {
                    skipped_bytes += c.len_utf8();
                    skipped_chars += 1;
                    true
                } else {
                    false
                }
            })
            .find(|&(_, (_, c))| (self.filter).is_char(c))
            .map(|(cidx, (bidx, _))| {
                let slice = &self.input[self.byte_offset + skipped_bytes..self.byte_offset + bidx];
                let tp = TokenPositional::convert_str(
                    slice,
                    self.char_offset + skipped_chars,
                    self.position,
                );
                self.byte_offset += bidx + 1;
                self.char_offset += cidx + 1;
                self.position += 1;
                tp
            })
            .or_else(|| {
                if self.byte_offset + skipped_bytes < self.input.len() {
                    let slice = &self.input[self.byte_offset + skipped_bytes..];
                    let tp = TokenPositional::convert_str(
                        slice,
                        self.char_offset + skipped_chars,
                        self.position,
                    );
                    self.byte_offset = self.input.len();
                    Some(tp)
                } else {
                    None
                }
            })
    }
}
impl<'a, F: CharFilter> Iterator for CharTokenSeqIter<'a, F> {
    type Item = TokenSequence<'a>;
    fn next(&mut self) -> Option<TokenSequence<'a>> {
        let mut skipped_bytes = 0;
        let mut skipped_chars = 0;

        //start from byte offset
        self.input[self.byte_offset..]
            .char_indices()
            .enumerate()
            .skip_while(|&(_, (_, c))| {
                if (self.filter).is_char(c) {
                    skipped_bytes += c.len_utf8();
                    skipped_chars += 1;
                    true
                } else {
                    false
                }
            })
            .find(|&(_, (_, c))| (self.filter).is_char(c))
            .map(|(cidx, (bidx, _))| {
                let slice = &self.input[self.byte_offset + skipped_bytes..self.byte_offset + bidx];
                let ts = TokenSequence::convert_str(slice);
                self.byte_offset += bidx + 1;
                self.char_offset += cidx + 1;
                self.position += 1;
                ts
            })
            .or_else(|| {
                if self.byte_offset + skipped_bytes < self.input.len() {
                    let slice = &self.input[self.byte_offset + skipped_bytes..];
                    let ts = TokenSequence::convert_str(slice);
                    self.byte_offset = self.input.len();
                    Some(ts)
                } else {
                    None
                }
            })
    }
}
impl<'a> TokenCmp<'a> {
    pub fn new_set(
        cow1: Vec<std::borrow::Cow<'a, str>>,
        cow2: Vec<std::borrow::Cow<'a, str>>,
    ) -> Self {
        TokenCmp {
            term1: Cow::from(cow1.iter().as_ref().concat()),
            term2: Cow::from(cow2.iter().as_ref().concat()),
        }
    }

    pub fn new_sort(
        mut cow1: Vec<std::borrow::Cow<'a, str>>,
        mut cow2: Vec<std::borrow::Cow<'a, str>>,
    ) -> Self {
        cow1.sort();
        cow2.sort();
        TokenCmp {
            term1: Cow::from(cow1.iter().as_ref().concat()),
            term2: Cow::from(cow2.iter().as_ref().concat()),
        }
    }

    pub fn new_set_join(
        cow1: Vec<std::borrow::Cow<'a, str>>,
        cow2: Vec<std::borrow::Cow<'a, str>>,
        sep: &str,
    ) -> Self {
        TokenCmp {
            term1: Cow::from(cow1.iter().as_ref().join(sep)),
            term2: Cow::from(cow2.iter().as_ref().join(sep)),
        }
    }
    pub fn new_sort_join(
        mut cow1: Vec<std::borrow::Cow<'a, str>>,
        mut cow2: Vec<std::borrow::Cow<'a, str>>,
        sep: &str,
    ) -> Self {
        cow1.sort();
        cow2.sort();
        TokenCmp {
            term1: Cow::from(cow1.iter().as_ref().join(sep)),
            term2: Cow::from(cow2.iter().as_ref().join(sep)),
        }
    }
    pub fn new_from_str(s1: &'a str, s2: &'a str) -> Self {
        TokenCmp {
            term1: s1.into(),
            term2: s2.into(),
        }
    }

    //RatcliffObserhelp distance; see matching_blocks
    fn longest_common_substring(
        shorter: &'a str,
        longer: &'a str,
        low1: usize,
        high1: usize,
        low2: usize,
        high2: usize,
    ) -> (usize, usize, usize) {
        let long_subtoken = &longer[low2..high2];
        let slen = high1 - low1;
        for size in (1..=slen).rev() {
            for start in 0..=slen - size {
                let short_subtoken = &shorter[low1 + start..low1 + start + size];
                let matches: Vec<(usize, &'a str)> =
                    long_subtoken.match_indices(short_subtoken).collect();
                // add sorting check...
                if let Some(&(startb, matchstr)) = matches.first() {
                    return (low1 + start, low2 + startb, matchstr.len());
                }
            }
        }
        (low1, low2, 0)
    }
}

impl<'a> TokenCmp<'a> {
    pub fn short_long_order_by_len(&self) -> (&str, &str) {
        let order = self.term1.cmp(&self.term2);
        match order {
            Ordering::Greater => (self.term2.as_ref(), self.term1.as_ref()),
            Ordering::Less => (self.term1.as_ref(), self.term2.as_ref()),
            Ordering::Equal => (self.term1.as_ref(), self.term2.as_ref()),
        }
    }
    // similarity...
    pub fn similarity(&self) -> u8 {
        //total length
        let sumlen = (self.term1.len() + self.term2.len()) as f32;
        //find the shorter and longer
        //iter, map, sum last block size
        let similar: usize = TokenCmp::matching_blocks(&self)
            .iter()
            .map(|&(_, _, s)| s)
            .sum();
        if sumlen > 0.0 {
            return (100.0 * (2.0 * (similar as f32) / sumlen)).round() as u8;
        }
        100
    }
    pub fn partial_similarity(&self) -> u8 {
        //find the shorter and longer
        //iter, map, sum last block size
        let blocks = TokenCmp::matching_blocks(&self);
        //set max
        let mut max: u8 = 0;
        // tuple through matching subsequence
        for (i, _, k) in blocks {
            let (short, _) = self.short_long_order_by_len();
            let sub = &short[::std::ops::Range {
                start: i,
                end: i + k,
            }];
            /*
            stack overflow
            let r = TokenCmp::partial_similarity(&TokenCmp {
                term1: Cow::from(short),
                term2: Cow::from(sub),
            });
            */
            let r = TokenCmp::similarity(&TokenCmp {
                term1: Cow::from(short),
                term2: Cow::from(sub),
            });
            if r > 99 {
                return 100;
            } else if r > max {
                max = r;
            }
        }
        max
    }
    /*
    RatcliffObserhelp distance
    // use this to replace matching_blocks
    fn matching_blocks() -> Vec<(usize, usize, usize)> {
        //https://github.com/matthieugomez/StringDistances.jl/blob/2834265e96f18993a98a57d97e9f27a450a161d1/src/distances/RatcliffObershelp.jl#L31
    */
    pub fn matching_blocks(&self) -> Vec<(usize, usize, usize)> {
        let (short, long) = &self.short_long_order_by_len();
        let (len1, len2) = (short.len(), long.len());
        let mut queue: Vec<(usize, usize, usize, usize)> = vec![(0, len1, 0, len2)];
        let mut m_blocks = Vec::new();
        while let Some((low1, high1, low2, high2)) = queue.pop() {
            let (i, j, k) =
                TokenCmp::longest_common_substring(short, long, low1, high1, low2, high2);
            if k != 0 {
                m_blocks.push((i, j, k));
                if low1 < i && low2 < j {
                    queue.push((low1, i, low2, j));
                }
                if i + k < high1 && j + k < high2 {
                    queue.push((i + k, high1, j + k, high2));
                }
            }
        }
        // sorting... m_blocks.sort()...
        let (mut i1, mut j1, mut k1) = (0, 0, 0);
        let mut non_adjacent = Vec::new();
        for (i2, j2, k2) in m_blocks {
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
}
impl TokenizerPosSummary {
    pub fn new(
        seq: String,
        seqln: usize,
        tstrings: Vec<String>,
        ofs: Vec<usize>,
        psns: Vec<usize>,
    ) -> Self {
        TokenizerPosSummary {
            sequence: seq,
            seqlen: seqln,
            token_strings: tstrings,
            offsets: ofs,
            positions: psns,
        }
    }
}
//Naive Methods... begin... these are 'naive' since they do not manage memory very well... moving back and forth from stack &str to heap String can be memory ineficient... recommend using these for smaller tasks on smaller data sets.
impl<'a> TokenizerNaive {
    /// words takes borrowed str and splits on pattern func filtering out empty slots; see tests.
    pub fn word_splitter(t: &str, pattern: &Fn(char) -> bool) -> Vec<String> {
        t.split(pattern)
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    }

    pub fn tokens_lower_with_filter(t: &str, pattern: &Fn(char) -> bool) -> String {
        t.split(pattern)
            .map(|s| s.to_lowercase())
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn tokens(t: &'a str) -> Vec<&'a str> {
        t.split("").filter(|s| !s.is_empty()).collect()
    }

    pub fn tokens_lower_str(s: &'a str) -> String {
        s.to_lowercase()
    }

    // remove all but alphanumeric characters
    pub fn tokens_alphanumeric(s: &'a str) -> String {
        s.chars()
            .map(|c| {
                if c.is_alphanumeric() {
                    c.to_string()
                } else {
                    ' '.to_string()
                }
            })
            .collect::<Vec<String>>()
            .concat()
    }

    pub fn pre_process(s: &'a str) -> String {
        TokenizerNaive::tokens_alphanumeric(&TokenizerNaive::tokens_lower_str(s))
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join(" ")
    }
}
impl<'a> TokenizerPos<'a> for AlphaNumericTokenizer {
    type TokenIter = CharTokenPosIter<'a, AlphaNumericCharFilter>;

    fn tokenize_pos(&self, input: &'a str) -> Self::TokenIter {
        CharTokenPosIter::new(AlphaNumericCharFilter, input)
    }

    fn pos_summary(&self, tokens: Vec<TokenPositional>) -> TokenizerPosSummary {
        let tlen = tokens.len();
        let mut tks: Vec<String> = Vec::with_capacity(tlen as usize);
        let mut ofs: Vec<usize> = Vec::with_capacity(tlen as usize);
        let mut psns: Vec<usize> = Vec::with_capacity(tlen as usize);

        for t in tokens {
            tks.push(t.term.to_lowercase());
            ofs.push(t.start_offset);
            psns.push(t.position)
        }

        TokenizerPosSummary::new(tks.join(" "), tks.len(), tks, ofs, psns)
    }
}
impl<'a> TokenizerSeq<'a> for AlphaNumericTokenizer {
    type TokenIter = CharTokenSeqIter<'a, AlphaNumericCharFilter>;

    fn tokenize_seq(&self, input: &'a str) -> Self::TokenIter {
        CharTokenSeqIter::new(AlphaNumericCharFilter, input)
    }

    fn sequencer(&self, input: &'a str) -> Vec<std::borrow::Cow<'a, str>> {
        let mut tks: Vec<std::borrow::Cow<'a, str>> = Vec::new();
        for t in self.tokenize_seq(input).collect::<Vec<TokenSequence<'a>>>() {
            tks.push(t.to_lower_cow());
        }
        tks
    }

    fn token(&self, input: &'a str) -> String {
        let mut tks: Vec<String> = Vec::new();
        for t in self.tokenize_seq(input).collect::<Vec<TokenSequence<'a>>>() {
            tks.push(t.term.to_lowercase());
        }
        tks.join(" ")
    }
}
impl<'a> TokenPositional<'a> {
    #[inline]
    pub fn convert_str(t: &'a str, start_offset: usize, position: usize) -> Self {
        TokenPositional {
            term: t.into(),
            start_offset: start_offset,
            position: position,
        }
    }
    pub fn to_lower_cow(&self) -> Cow<'a, str> {
        self.term.to_lowercase().into()
    }
}
impl<'a> TokenSequence<'a> {
    #[inline]
    pub fn convert_str(t: &'a str) -> Self {
        TokenSequence { term: t.into() }
    }
    pub fn to_lower_cow(&self) -> Cow<'a, str> {
        self.term.to_lowercase().into()
    }
}
impl<'a> std::fmt::Display for TokenPositional<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}, {}, {}", self.term, self.start_offset, self.position)
    }
}
impl<'a> std::fmt::Display for TokenSequence<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.term)
    }
}
impl<T> U8IterExt for T
where
    T: Iterator<Item = u8>,
{
    fn u8_max(&mut self) -> u8 {
        self.fold(0, u8::max)
    }

    fn u8_min(&mut self) -> u8 {
        self.fold(0, u8::min)
    }
}

//////////////////////////////////////////////////////////////////////
//////////// TESTs
//////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    fn word_split(c: char) -> bool {
        match c {
            '\n' | '|' | '-' => true,
            _ => false,
        }
    }
    fn tokens_filter(c: char) -> bool {
        match c {
            '-' | '|' | '*' | ')' | '(' | '&' => true,
            _ => false,
        }
    }

    // fn ant() -> AlphaNumericTokenizer {
    //     AlphaNumericTokenizer
    // }

    #[test]
    fn tokenizer_sequencer() {
        let an = AlphaNumericTokenizer;
        let one = an.sequencer("Marriot &Beaches Resort|").join(" ");
        let two = an.sequencer("Marriot& Beaches^ Resort").join(" ");
        assert_eq!(one, two);
    }

    #[test]
    fn on_pre_process() {
        let res = TokenizerNaive::pre_process("Hotel & Ristorante Bellora");
        assert_eq!(res, "hotel ristorante bellora");

        let res1 = TokenizerNaive::pre_process("Auténtico Hotel");
        assert_eq!(res1, "auténtico hotel");

        let res2 = TokenizerNaive::pre_process("Residence Chalet de l'Adonis");
        assert_eq!(res2, "residence chalet de l adonis");

        let res6 = TokenizerNaive::pre_process("HOTEL EXCELSIOR");
        assert_eq!(res6, "hotel excelsior");

        let res6 = TokenizerNaive::pre_process("Kotedzai Trys pusys,Pylimo ");
        assert_eq!(res6, "kotedzai trys pusys pylimo");

        let res6 = TokenizerNaive::pre_process("Inbursa Cancún Las Américas");
        assert_eq!(res6, "inbursa cancún las américas");
    }

    #[test]
    fn on_tokens_alphanumeric() {
        let res3 = TokenizerNaive::tokens_alphanumeric("|HelLo tHere");
        assert_eq!(res3, " HelLo tHere");

        let res4 = TokenizerNaive::tokens_alphanumeric("HelLo|tHere");
        assert_eq!(res4, "HelLo tHere");

        let res5 = TokenizerNaive::tokens_alphanumeric("HelLo * & )(tHere");
        assert_eq!(res5, "HelLo       tHere");
    }

    #[test]
    fn on_tokens_lower() {
        let res = TokenizerNaive::tokens_lower_str("HelLo tHerE");
        assert_eq!(res, "hello there")
    }

    #[test]
    fn on_tokens_simple() {
        assert_eq!(
            TokenizerNaive::tokens("hello there"),
            ["h", "e", "l", "l", "o", " ", "t", "h", "e", "r", "e"]
        )
    }

    #[test]
    fn on_tokens_lower_filter() {
        let res = TokenizerNaive::tokens_lower_with_filter("|HelLo tHere", &tokens_filter);
        assert_eq!(res, " hello there");

        let res1 = TokenizerNaive::tokens_lower_with_filter("HelLo|tHere", &tokens_filter);
        assert_eq!(res1, "hello there");

        let res2 = TokenizerNaive::tokens_lower_with_filter("HelLo tHere", &tokens_filter);
        assert_eq!(res2, "hello there");

        let res6 =
            TokenizerNaive::tokens_lower_with_filter("****HelLo *() $& )(tH*ere", &tokens_filter);
        assert_eq!(res6, "    hello     $    th ere");
    }

    #[test]
    fn on_word_splitter() {
        let res = TokenizerNaive::word_splitter("HelLo|tHere", &word_split);
        assert_eq!(res, vec!["HelLo", "tHere"])
    }

    #[test]
    fn on_similarity_identity() {
        let t = TokenCmp::new_from_str("hello", "hello");
        assert_eq!(t.similarity(), 100);
    }

    #[test]
    fn on_similarity_high() {
        let t = TokenCmp::new_from_str("hello b", "hello");
        assert_eq!(t.similarity(), 83);
    }

    #[test]
    fn on_token_sort() {
        let s1 = "Marriot Beaches Resort foo";
        let s2 = "Beaches Resort Marriot bar";
        assert_eq!(TokenCmp::new_from_str(s1, s2).similarity(), 62);
        let sim = token_sort(s1, s2, &TokenCmp::new_sort, &TokenCmp::similarity);
        assert_eq!(sim, 87);
    }

    #[test]
    fn on_amstel_match_for_nate() {
        let sabre = "INTERCONTINENTAL AMSTEL AMS";
        let ean = "InterContinental Amstel Amsterdam";
        assert_eq!(TokenCmp::new_from_str(sabre, ean).similarity(), 20);
        assert_eq!(TokenCmp::new_from_str(sabre, ean).partial_similarity(), 14);
        assert_eq!(
            token_sort(sabre, ean, &TokenCmp::new_sort, &TokenCmp::similarity),
            79
        );

        assert_eq!(
            token_sort(
                sabre,
                ean,
                &TokenCmp::new_sort,
                &TokenCmp::partial_similarity
            ),
            78
        );
    }

    #[test]
    fn on_partial_similarity_identity() {
        let t = TokenCmp::new_from_str("hello", "hello");
        assert_eq!(t.partial_similarity(), 100);
    }

    #[test]
    fn on_partial_similarity_high() {
        let t = TokenCmp::new_from_str("hello b", "hello");
        assert_eq!(t.partial_similarity(), 100);
    }

    #[test]
    fn on_similarity_and_whitespace_difference() {
        let t1 = TokenCmp::new_from_str("hello bar", "hello");
        let t2 = TokenCmp::new_from_str("hellobar", "hello");
        let sim1 = t1.similarity();
        let sim2 = t2.similarity();
        assert_ne!(sim1, sim2);
        assert!(sim1 < sim2);
        assert_eq!(sim1, 71);
        assert_eq!(sim2, 77);
    }

    /*

    #[test]
    fn case_insensitive() {
        assert!(similarity("hello WORLD", "hello world") != 100);
        assert_eq!(
            similarity(
                &TokenizerNaive::pre_process("hello WORLD"),
                &TokenizerNaive::pre_process("hello world")
            ),
            100
        );
    }

    #[test]
    fn token_sort() {
        assert_eq!(
            token_sort_similarity("Marriot Beaches Resort", "Beaches Resort Marriot"),
            100
        );
    }

    #[test]
    fn token_sort_with_sequencer() {
        let an = AlphaNumericTokenizer;
        assert_eq!(
            token_sort_similarity(
                &an.sequencer("Marriot** Beaches ^ Resort").join(" "),
                &an.sequencer("Beaches Resort^ (Marriot").join(" "),
            ),
            100
        );
    }

    #[test]
    fn partial() {
        assert_eq!(partial_similarity("hello", "hello world"), 100);
    }

    #[test]
    fn partial_token_sort() {
        assert_eq!(
            partial_token_set_similarity(
                "new york mets vs atlanta braves",
                "atlanta braves vs new york mets",
            ),
            100
        );
    }

    #[test]
    fn token_set() {
        assert_eq!(
            token_set_similarity(
                "new york mets vs atlanta braves",
                "atlanta braves vs new york mets",
            ),
            100
        );
    }

    #[test]
    fn partial_token_set() {
        assert_eq!(
            partial_token_set_similarity(
                "new york mets vs atlanta braves",
                "new york city mets - atlanta braves",
            ),
            100
        );
    }
    */
}
