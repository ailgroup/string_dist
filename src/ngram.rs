use crate::cosine::{Qgram, QgramVec};
use array_tool::vec::{Intersect, Union};
use std::borrow::Cow;

// REFACTOR ngram to deal with str lifetime Cow

/// Ngram is continuous sequence of n-items from a given sequence. The distance is the relative number of items between these two sequences.
///
/// References:
///
///	* [N-Gram Similarity and Distance](https://webdocs.cs.ualberta.ca/~kondrak/papers/spire05.pdf)
///	* [Wikipedia n-gram](https://en.wikipedia.org/wiki/N-gram)
///	* [WolframAlpha n-gram](http://m.wolframalpha.com/input/?i=n-grams+%22n-gram+example+of+n-grams+in+wolfram+alpha%array_tool = "~1.0.3"&x=0&y=0)
pub struct NGram<'a> {
    pub n: usize,
    pub sv1: Vec<Cow<'a, str>>,
    pub sv2: Vec<Cow<'a, str>>,
    pub sv1_len: usize,
    pub sv2_len: usize,
    pub intersect: Vec<Cow<'a, str>>,
    pub union: Vec<Cow<'a, str>>,
    pub intersect_len: usize,
    pub union_len: usize,
    pub qgram: Qgram,
}

impl<'a> NGram<'a> {
    pub fn from_str(string1: &'a str, string2: &'a str, n: usize) -> Self {
        let mut sv1: Vec<std::borrow::Cow<'a, str>> = Vec::new();
        let mut sv2: Vec<std::borrow::Cow<'a, str>> = Vec::new();

        for i in 0..=string1.len() - n {
            sv1.push(Cow::from(&string1[i..(i + n)]));
        }
        for i in 0..=string2.len() - n {
            sv2.push(Cow::from(&string2[i..(i + n)]));
        }

        let sa1_len = sv1.len();
        let sa2_len = sv2.len();
        let intersect = sv1.intersect(sv2.clone());
        let union = sv1.union(sv2.clone());
        let int_len = intersect.len();
        let un_len = union.len();

        let mut qv1: Vec<f64> = Vec::with_capacity(un_len as usize);
        let mut qv2: Vec<f64> = Vec::with_capacity(un_len as usize);
        for c in &union {
            if sv1.contains(c) {
                qv1.push(1.0);
            } else {
                qv1.push(0.0);
            }

            if sv2.contains(c) {
                qv2.push(1.0);
            } else {
                qv2.push(0.0);
            }
        }

        NGram {
            n: n,
            sv1: sv1,
            sv2: sv2,
            sv1_len: sa1_len,
            sv2_len: sa2_len,
            intersect: intersect,
            union: union,
            intersect_len: int_len,
            union_len: un_len,
            qgram: Qgram {
                a: QgramVec::from_vec(qv1),
                b: QgramVec::from_vec(qv2),
            },
        }
    }
}

impl<'a> NGram<'a> {
    /// jaccard_distance: 1 - jaccard_similarity. higher score is less similar.
    pub fn jaccard_distance(&self) -> f64 {
        1.0 - NGram::jaccard_similarity(&self)
    }

    /// cosine_distance: 1 - cosine_similarity. higher score is less similar.
    pub fn cosine_distance(&self) -> f64 {
        1.0 - NGram::cosine_similarity(&self)
    }

    /// jaccard_similarity: calculates jaccard coefficient, the similarity
    /// of two sets as intersection divided by union:
    /// ``` J(X,Y) = |X∩Y| / |X∪Y| ```
    /// higher score is more similar
    pub fn jaccard_similarity(&self) -> f64 {
        ((self.intersect_len as f32) / (self.union_len as f32)).into()
    }

    /// cosine_similarity: higher score is more similar
    pub fn cosine_similarity(&self) -> f64 {
        let a = &self.qgram.a;
        let b = &self.qgram.b;
        (a * b).sum() / (((a * a).sum()).sqrt() * ((b * b).sum()).sqrt())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ngram_ceaser() {
        let b1 = NGram::from_str("Ceasers Palace", "Caesars Palace", 4);
        assert_eq!((b1.jaccard_similarity() as u64), (0.375 as u64));
        assert_eq!(
            (b1.cosine_similarity() as u64),
            (0.545_454_545_454_545_4 as u64)
        );
        assert_eq!((b1.jaccard_distance() as u64), (0.625 as u64));
        assert_eq!(
            (b1.cosine_distance() as u64),
            (0.454_545_454_545_454_6 as u64)
        );
    }

    #[test]
    fn on_ngram_hello() {
        let b2 = NGram::from_str("Hello, world!", "Hello, porld!", 2);
        assert_eq!(
            (b2.jaccard_similarity() as u64),
            (0.714_285_714_285_714_3 as u64)
        );
        assert_eq!(
            (b2.cosine_similarity() as u64),
            (0.833_333_333_333_333_5 as u64)
        );
        assert_eq!(
            (b2.jaccard_distance() as u64),
            (0.285_714_285_714_285_7 as u64)
        );
        assert_eq!(
            (b2.cosine_distance() as u64),
            (0.166_666_666_666_666_5 as u64)
        );
    }

}
