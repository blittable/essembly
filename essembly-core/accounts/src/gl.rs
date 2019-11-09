use mockall::predicate::*;
use mockall::*;

#[automock]
trait MyTrait {
    fn foo(&self, x: u32) -> u32;
}

#[test]
fn t_test() {
    let mut mock = MockMyTrait::new();
    mock.expect_foo().with(eq(4)).times(1).returning(|x| x + 1);
    assert_eq!(5, mock.foo(4));
}
