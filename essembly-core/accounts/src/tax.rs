
#[cfg(test)]
use mockall::*;

trait Taxable {
    fn calc_tax(&self, amount: f32, rate: f32) -> f32;
}

#[cfg_attr(test, automock)]
impl Taxable {
pub fn calc_tax(&self, amount: f32, rate: f32) -> f32 {
    amount * rate
}

}

mod test {
    #![allow(unused_imports)]
    use super::*;
    use crate::prelude::*;
    #[test]
    pub fn calc_default_tax_impl() {
        #[allow(unused_variables)]
        let mut mock = MockTaxable::new();
        mock.expect_calc_tax()
        .times(1)
        .returning(|x, y| x * y);

        assert_eq!(2.0, mock.calc_tax(10.0, 0.2));
    }
}

