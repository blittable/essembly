/*!
# Operation 
An [`Operation`] represents an action taken by a User in the system. 

*/ 

use uuid::Uuid;
use uuid::{Variant, Version};

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum OperationTypes {
    SYSTEM,
    ADMINISTRATIVE,
    GROUP_ADMINSTRATIVE,
    USER,
    GUEST,
}

#[derive(Debug)]
pub struct Id {
    pub id:  OperationTypes,
}

trait Operation<T> {
    type id;
    fn new() -> Self;
    fn run(&self) -> Result<T>;
    fn get_id(&self) -> Id;
}

pub struct Operation {}

impl<T> Operation<T> for Operation {

    type id = OperationTypes;

    fn new() -> Option<T> {
        Operation {}
    }

    fn get_id(&self) -> Id {
        &self.id
    }

    fn set_id(&mut self, id: OperationTypes) -> &Ids<T> {
        self.id = id;
        &self.id
    }
}


// #[test]
// fn build_user() {
//     //We have variants of IDs to test:
//     //A) Internal - GUID
//     //B) External - Any
//     let ext_id = ExternalId {
//         id: "John".to_string(),
//     };
//     let i = Identifier { id_type: ext_id };

//     let j = User {
//         id: Ids::Internal(i),
//         permissions: Permissions::new(),
//     };

//     let guid_id = InternalId {
//         id: Uuid::new_v5(&Uuid::NAMESPACE_X500, "foo".as_bytes()),
//     };
//     let k = Identifier { id_type: guid_id };

//     let m = User {
//         id: Ids::Internal(k),
//         permissions: Permissions::new(),
//     };
// }

// #[test]
// fn build_object_test() {
//     let innertype = Businesses::LLC;
//     let b = Object {
//         object_type: ObjectType::Business(innertype),
//     };
//     println!("Object Type: {:?}", b);

//     //println!("Object Type: {:?}", b.1);
//     //assert_eq!(b.object_type, Businesses::LLC);
// }
