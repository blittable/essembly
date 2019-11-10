use uuid::Uuid;

#[cfg(test)]
use mockall::*;

#[cfg_attr(test, automock)]
pub trait Clocked {
    fn clock_in(&self, id: Uuid) -> Result<(), u32>;
    fn clock_out(&self, id: uuid::Version) -> u32;
}

impl dyn Clocked {
    #[allow(dead_code)]
    #[allow(unused_variables)]
    fn clock_in(&self, id: Uuid) -> u32 {
        4
    }

    #[allow(dead_code)]
    #[allow(unused_variables)]
    fn clock_out(&self, id: uuid::Version) -> u32 {
        5
    }
}

mod test {
    #![allow(unused_imports)]
    use super::*;
    use crate::prelude::*;
    #[test]
    pub fn clocked_test() {
        #[allow(unused_variables)]
        let mock = MockClocked::new();
        let uuid: Uuid = uuid::Uuid::new_v5(&Uuid::NAMESPACE_X500, "Adam Smith".as_bytes());
        println!("UUID: {}", uuid);
        //mock.expect_clock_in().with(eq("Adam Smith")).return_const("90d39c96-9b46-5feb-a8c3-87bb389ce3e3");
        //assert_eq!(Uuid::fromStr("90d39c96-9b46-5feb-a8c3-87bb389ce3e3"), mock.clock_in(uuid));

        assert_eq!(uuid.get_variant(), Some(Variant::RFC4122));
        assert_eq!(uuid.get_variant(), Some(Variant::RFC4122));
        assert_eq!(uuid.get_version(), Some(Version::Sha1));
        println!("UUID: {}", uuid);
    }
}
