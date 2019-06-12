# string_dist
Work in progress. Provides string distance algos. This is a custom project to support specific needs in a "product matching" data processing framework as well as for supporting algos in a back-office insurance form OCR project.

### A Note
This package contains alot of refactored or re-implemented string distance algorithms found in the existing rust ecosystem. I've tried to point out where the original ideas or implementations came from. 

The rational for not doing pull requests to original projects is twofold:

1. my changes were often so pervasive that a PR would probably have been rude
2. I needed (or will need) some common interfaces or inter-functionality between algos and maintaining a meta-package of forked projects that were all but gutted in many places did not make sense.

### Edit distances char-based

* Hamming
* Jaro
* Jaro-Winkler

#### todo
* Mong Elken

### Token Based metrics

* NGram
* QGram
* Jaccard
* Cosine

#### todo

* TFIDF
* Jensen-Shannon

### Maximum Likelihood Estimation for Ngram Models
#### todo
yes, see if the NGram token comparison can fit into an NGram model with log-likelihood and MLE...>

