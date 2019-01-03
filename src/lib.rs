pub mod hamming;
pub mod ngram;
pub mod tokenize;

#[cfg(test)]
mod tests {
    use super::ngram;
    #[test]
    fn ngram_basic() {
        let abc = "abcde";
        let abd = "abdcde";
        let n = ngram::build(abc, abd, 2);
        assert_eq!(n.s1_len, 4.0);
        assert_eq!(n.s2_len, 5.0);
        assert_eq!(n.union_len, 6.0);
        assert_eq!(n.intersect_len, 3.0);
        assert_eq!(n.s1, vec!["ab", "bc", "cd", "de"]);
        assert_eq!(n.s2, vec!["ab", "bd", "dc", "cd", "de"]);
        assert_eq!(n.qgram.q1, vec![1, 1, 1, 1, 0, 0]);
        assert_eq!(n.qgram.q2, vec![1, 0, 1, 1, 1, 1]);
        assert_eq!(n.jaccard_similarity(), 0.5);
        assert_eq!(n.cosine_similarity(), 0.6708203932499369);
        assert_eq!(n.jaccard_distance(), 0.5);
        assert_eq!(n.cosine_distance(), 0.3291796067500631);
    }
}
