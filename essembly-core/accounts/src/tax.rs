#[cfg(test)]
use mockall::*;

#[cfg_attr(test, automock)]
trait Taxable {
    fn calc_tax(&self, amount: f32, rate: f32) -> f32;
}

impl dyn Taxable {
    pub fn calc_tax(&self, amount: f32, rate: f32) -> f32 {
        amount * rate
    }
}

mod test {
    #![allow(unused_imports)]
    #![allow(unused_variables)]
    use super::*;
    #[test]
    pub fn calc_default_tax_impl() {
        let mut mock = MockTaxable::new();
        mock.expect_calc_tax().times(1).returning(|x, y| x * y);

        assert_eq!(2.0, mock.calc_tax(10.0, 0.2));
    }
}
