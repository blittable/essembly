use crate::prelude::*;
use uuid::Uuid;

#[automock]
pub trait Clocked {
    fn clock_in(&self, id: Uuid) -> String;
    fn clock_out(&self, id: uuid::Version) -> u32;
}

impl Clocked {
    fn clock_in(&self, id: Uuid) -> u32 {
        4
    }
    fn clock_out(&self, id: uuid::Version) -> u32 {
        5
    }
}


#[test]
pub fn clocked_test() {
    use crate::prelude::*;

    let mut mock = MockClocked::new();
    let uuid: Uuid = uuid::Uuid::new_v5(&Uuid::NAMESPACE_X500, "Adam Smith".as_bytes());
    println!("UUID: {}", uuid);
    //mock.expect_clock_in().with(eq("Adam Smith")).return_const("90d39c96-9b46-5feb-a8c3-87bb389ce3e3");

    //assert_eq!(Uuid::fromStr("90d39c96-9b46-5feb-a8c3-87bb389ce3e3"), mock.clock_in(uuid));

    assert_eq!(uuid.get_variant(), Some(Variant::RFC4122));
    assert_eq!(uuid.get_variant(), Some(Variant::RFC4122));
    assert_eq!(uuid.get_version(), Some(Version::Sha1));
    println!("UUID: {}", uuid);
}
