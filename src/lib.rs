pub mod cosine;
pub mod hamming;
pub mod jaro_winkler;
pub mod ngram;
pub mod tokenize;
#[allow(clippy::float_cmp)]
#[cfg(test)]
mod tests {
    use super::{ngram::NGram, tokenize};
    use crate::tokenize::TokenizerSeq;

    // REFACTOR ngram to deal with str lifetime Cow
    #[test]
    fn on_ngram_amstel_match_for_nate() {
        let sabre = "INTERCONTINENTAL AMSTEL AMS";
        let ean = "InterContinental Amstel Amsterdam";
        let an = tokenize::AlphaNumericTokenizer;
        let t_sabre = &an.token(sabre)[..];
        let t_ean = &an.token(ean)[..];
        let b2 = NGram::from_str(t_sabre, t_ean, 4);
        assert!(b2.jaccard_similarity() > 0.84);
        assert!(b2.cosine_similarity() > 0.919_866);
        assert!(b2.jaccard_distance() > 0.153_846);
        assert!(b2.cosine_distance() > 0.080_133);
    }

    /*
    #[test]
    fn ngram_basic() {
        let abc = "abcde";
        let abd = "abdcde";
        let n = ngram::build(abc, abd, 2);
        // assert_eq!(n.sv1_len, (f64::4.0).into());
        assert_eq!(n.sv2_len, 5.0);
        assert_eq!(n.union_len, f64::from(6.0));
        assert_eq!(n.intersect_len, f64::from(3.0));
        assert_eq!(n.sv1, vec!["ab", "bc", "cd", "de"]);
        assert_eq!(n.sv2, vec!["ab", "bd", "dc", "cd", "de"]);
        //assert_eq!(n.qgram.q1, vec![1, 1, 1, 1, 0, 0]);
        //assert_eq!(n.qgram.q2, vec![1, 0, 1, 1, 1, 1]);

        assert_eq!(
            n.qgram.a,
            QgramVec::from_vec(vec![1.0, 1.0, 1.0, 1.0, 0.0, 0.0])
        );
        assert_eq!(
            n.qgram.b,
            QgramVec::from_vec(vec![1.0, 0.0, 1.0, 1.0, 1.0, 1.0])
        );

        assert_eq!(n.jaccard_similarity(), f64::from(0.5));
        assert_eq!(n.cosine_similarity(), f64::from(0.6708203932499369));
        assert_eq!(n.jaccard_distance(), f64::from(0.5));
        assert_eq!(n.cosine_distance(), f64::from(0.3291796067500631));
    }
    */
}
