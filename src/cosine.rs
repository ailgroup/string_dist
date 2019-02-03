use ndarray::Array1;

pub trait Cosine {
    fn similarity(&self) -> f64;
}

pub type QgramVec = Array1<f64>;
pub struct Qgram {
    pub a: QgramVec,
    pub b: QgramVec,
}

impl Cosine for Qgram {
    fn similarity(&self) -> f64 {
        let a = &self.a;
        let b = &self.b;
        (a * b).sum() / (((a * a).sum()).sqrt() * ((b * b).sum()).sqrt())
    }
}

impl Cosine for (Array1<f64>, Array1<f64>) {
    fn similarity(&self) -> f64 {
        let (a, b) = (&self.0, &self.1);
        (a * b).sum() / (((a * a).sum()).sqrt() * ((b * b).sum()).sqrt())
    }
}
