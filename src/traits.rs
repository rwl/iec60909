pub(crate) trait Sq {
    fn sq(&self) -> Self;
}

impl Sq for f64 {
    fn sq(&self) -> f64 {
        self.powi(2)
    }
}
