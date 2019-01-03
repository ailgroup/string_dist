use array_tool::vec::{Intersect, Union};
/// Ngram is continuous sequence of n-items from a given sequence. The distance is the relative number of items between these two sequences.
///
/// References:
///
///	[N-Gram Similarity and Distance](https://webdocs.cs.ualberta.ca/~kondrak/papers/spire05.pdf)
///	[Wikipedia n-gram](https://en.wikipedia.org/wiki/N-gram)
///	[WolframAlpha n-gram](http://m.wolframalpha.com/input/?i=n-grams+%22n-gram+example+of+n-grams+in+wolfram+alpha%22&x=0&y=0)

pub struct NGram {
    pub s1: Vec<String>,
    pub s2: Vec<String>,
    pub s1_len: f64,
    pub s2_len: f64,
    pub intersect: Vec<String>,
    pub union: Vec<String>,
    pub intersect_len: f64,
    pub union_len: f64,
    pub qgram: Qgram,
}
pub struct Qgram {
    pub q1: Vec<usize>,
    pub q2: Vec<usize>,
}

pub fn build(string1: &str, string2: &str, n: usize) -> NGram {
    let mut tsg = NGram {
        s1: Vec::with_capacity(string1.len() as usize),
        s2: Vec::with_capacity(string2.len() as usize),
        s1_len: 0.0,
        s2_len: 0.0,
        intersect: Vec::new(),
        union: Vec::new(),
        intersect_len: 0.0,
        union_len: 0.0,
        qgram: Qgram {
            q1: Vec::new(),
            q2: Vec::new(),
        },
    };

    for i in 0..((string1.len() - n) + 1) {
        tsg.s1.push(string1[i..(i + n)].to_string());
    }
    for i in 0..((string2.len() - n) + 1) {
        tsg.s2.push(string2[i..(i + n)].to_string());
    }

    tsg.s1_len = tsg.s1.len() as f64;
    tsg.s2_len = tsg.s2.len() as f64;
    tsg.intersect = tsg.s1.intersect(tsg.s2.clone());
    tsg.union = tsg.s1.union(tsg.s2.clone());
    tsg.intersect_len = tsg.intersect.len() as f64;
    tsg.union_len = tsg.union.len() as f64;

    let cap = tsg.union_len as usize;
    tsg.qgram.q1 = Vec::with_capacity(cap);
    tsg.qgram.q2 = Vec::with_capacity(cap);
    for c in &tsg.union {
        if tsg.s1.contains(c) {
            tsg.qgram.q1.push(1)
        } else {
            tsg.qgram.q1.push(0)
        }
        if tsg.s2.contains(c) {
            tsg.qgram.q2.push(1)
        } else {
            tsg.qgram.q2.push(0)
        }
    }

    tsg
}

impl NGram {
    /// jaccard_distance: 1 - jaccard_similarity. higher score is less similar.
    pub fn jaccard_distance(&self) -> f64 {
        1.0 - NGram::jaccard_similarity(&self)
    }
    /// cosine_distance: 1 - cosine_similarity. higher socre is less similar.
    pub fn cosine_distance(&self) -> f64 {
        1.0 - NGram::cosine_similarity(&self)
    }

    /// jaccard_similarity: calculates jaccard coefficient, the similarity
    /// of two sets as intersection divided by union: J(X,Y) = |X∩Y| / |X∪Y|.
    /// higher score is more similar
    pub fn jaccard_similarity(&self) -> f64 {
        (&self.intersect_len / &self.union_len)
    }
    /// cosine_similarity:
    /// higher score is more similar
    pub fn cosine_similarity(&self) -> f64 {
        let s1s1_mul: f64 = self
            .qgram
            .q1
            .iter()
            .zip(self.qgram.q1.iter())
            .map(|(x, y)| (x * y) as f64)
            .sum();
        let s1s2_mul: f64 = self
            .qgram
            .q1
            .iter()
            .zip(self.qgram.q2.iter())
            .map(|(x, y)| (x * y) as f64)
            .sum();
        let s2s2_mul: f64 = self
            .qgram
            .q2
            .iter()
            .zip(self.qgram.q2.iter())
            .map(|(x, y)| (x * y) as f64)
            .sum();
        s1s2_mul / (s1s1_mul.sqrt() * s2s2_mul.sqrt())
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
