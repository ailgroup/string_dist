use crate::cosine::{Qgram, QgramVec};
use array_tool::vec::{Intersect, Union};

/// Ngram is continuous sequence of n-items from a given sequence. The distance is the relative number of items between these two sequences.
///
/// References:
///
///	* [N-Gram Similarity and Distance](https://webdocs.cs.ualberta.ca/~kondrak/papers/spire05.pdf)
///	* [Wikipedia n-gram](https://en.wikipedia.org/wiki/N-gram)
///	* [WolframAlpha n-gram](http://m.wolframalpha.com/input/?i=n-grams+%22n-gram+example+of+n-grams+in+wolfram+alpha%22&x=0&y=0)
///
pub struct NGram {
    pub n: usize,
    pub string1: String,
    pub string2: String,
    pub sv1: Vec<String>,
    pub sv2: Vec<String>,
    pub sv1_len: f64,
    pub sv2_len: f64,
    pub intersect: Vec<String>,
    pub union: Vec<String>,
    pub intersect_len: f64,
    pub union_len: f64,
    pub qgram: Qgram,
}

/*
pub type QgramVec = Array1<f64>;
pub struct Qgram {
    pub q1: QgramVec,
    pub q2: QgramVec,
}
*/

pub fn build(string1: &str, string2: &str, n: usize) -> NGram {
    let mut sa1 = Vec::with_capacity(string1.len() as usize);
    let mut sa2 = Vec::with_capacity(string2.len() as usize);

    for i in 0..((string1.len() - n) + 1) {
        sa1.push(string1[i..(i + n)].to_string());
    }
    for i in 0..((string2.len() - n) + 1) {
        sa2.push(string2[i..(i + n)].to_string());
    }

    let sa1_len = sa1.len() as f64;
    let sa2_len = sa2.len() as f64;
    let intersect = sa1.intersect(sa2.clone());
    let union = sa1.union(sa2.clone());
    let int_len = intersect.len() as f64;
    let un_len = union.len() as f64;

    let mut qv1: Vec<f64> = Vec::with_capacity(un_len as usize);
    let mut qv2: Vec<f64> = Vec::with_capacity(un_len as usize);
    for c in &union {
        if sa1.contains(c) {
            qv1.push(1.0);
        } else {
            qv1.push(0.0);
        }

        if sa2.contains(c) {
            qv2.push(1.0);
        } else {
            qv2.push(0.0);
        }
    }

    NGram {
        n: n,
        string1: string1.to_string(),
        string2: string2.to_string(),
        sv1: sa1,
        sv2: sa2,
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

impl NGram {
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
        (&self.intersect_len / &self.union_len)
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
    use super::build;
    #[test]
    fn table_ngram() {
        struct Table {
            one: String,
            two: String,
            jaccard_sim_expect: f64,
            cosine_sim_expect: f64,
            jaccard_dist_expect: f64,
            cosine_dist_expect: f64,
            n: usize,
        }
        fn build_table(
            one: String,
            two: String,
            jaccard_sim_expect: f64,
            cosine_sim_expect: f64,
            jaccard_dist_expect: f64,
            cosine_dist_expect: f64,
            n: usize,
        ) -> Table {
            Table {
                one,
                two,
                jaccard_sim_expect,
                cosine_sim_expect,
                jaccard_dist_expect,
                cosine_dist_expect,
                n,
            }
        }
        let table = vec![
            build_table(
                String::from("Ceasers Palace"),
                String::from("Caesars Palace"),
                0.375,
                0.5454545454545454,
                0.625,
                0.4545454545454546,
                4,
            ),
            build_table(
                String::from("Hello, world!"),
                String::from("Hello, porld!"),
                0.7142857142857143,
                0.8333333333333335,
                0.2857142857142857,
                0.16666666666666652,
                2,
            ),
        ];

        for t in table {
            let n = build(&t.one, &t.two, t.n);
            assert_eq!(n.jaccard_similarity(), t.jaccard_sim_expect);
            assert_eq!(n.cosine_similarity(), t.cosine_sim_expect);
            assert_eq!(n.jaccard_distance(), t.jaccard_dist_expect);
            assert_eq!(n.cosine_distance(), t.cosine_dist_expect);
        }
    }
}
