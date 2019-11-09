#[cfg(test)]
use mockall::*;

#[cfg_attr(test, automock)]
trait MyTrait {
    fn foo(&self, x: u32) -> u32;
}

#[cfg(test)]
mod test {
    #![allow(unused_imports)]
    use super::*;
    use crate::prelude::*;
    #[test]
    fn test_x() {
        #[allow(unused_variables)]
        let mut mock = MockMyTrait::new();
        mock.expect_foo()
            .with(mockall::predicate::eq(4))
            .times(1)
            .returning(|x| x + 1);
        assert_eq!(5, mock.foo(4));
    }
}
