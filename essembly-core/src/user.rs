/*!
# User
A [`User`] represents a person interacting with the Essembly system.

*/ 

use crate::permissions::Permissions;
use mockall::predicate::str;
use mockall::*;
use uuid::Uuid;
use uuid::{Variant, Version};

/// ExternalId is used when a user as an id that is outside the system
/// (e.g. employee number)
/// # Example
#[derive(Debug)]
pub struct ExternalId {
    id: String,
}

#[derive(Debug)]
pub struct InternalId {
    pub id: Uuid,
}


#[derive(Debug)]
pub struct Identifier<T> {
    pub id_type: T,
}

#[derive(Debug)]
pub enum Ids<T> {
    Internal(Identifier<T>),
    External(Identifier<T>),
}

///!The Id form to be used in the system, generic over the type of Id
trait Id<T> {
    fn get_id(&self) -> &Ids<T>;
    fn set_id(&mut self, id: Ids<T>) -> &Ids<T>;
}

impl<T> Id<T> for User<T> {
    fn get_id(&self) -> &Ids<T> {
        &self.id
    }

    fn set_id(&mut self, id: Ids<T>) -> &Ids<T> {
        self.id = id;
        &self.id
    }
}

#[doc(inline)]
///!The permissionable person in the system
#[derive(Debug)]
struct User<T> {
    pub(crate) id: Ids<T>,
    pub(crate) permissions: Permissions,
}

impl<T> User<T> {
    fn set_permissions(&mut self, permissions: Permissions) {
        self.permissions = permissions;
    }
    fn get_permissions(&self) -> &Permissions {
        &self.permissions
    }
}

///!Linkable is an abstraction over a link between composables
#[doc(inline)]
trait Join<T>: FnOnce() -> T {}
impl<T, U> Join<T> for U where U: FnOnce() -> T {}


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ObjectType {
    Record,
    Module,
}

#[derive(Debug)]
///!The object against which permissions can be applied
struct Object {
    object_type: ObjectType,
}

impl Object {
    fn new(&self, _object_type: ObjectType) -> Self {
        return Self {
            object_type: _object_type,
        };
    }
}

#[test]
fn build_user() {
    //We have variants of IDs to test:
    //A) Internal - GUID
    //B) External - Any
    let ext_id = ExternalId {
        id: "John".to_string(),
    };
    let i = Identifier { id_type: ext_id };

    let _j = User {
        id: Ids::Internal(i),
        permissions: Permissions::new(),
    };

    let guid_id = InternalId {
        id: Uuid::new_v5(&Uuid::NAMESPACE_X500, "foo".as_bytes()),
    };
    let k = Identifier { id_type: guid_id };

    let _m = User {
        id: Ids::Internal(k),
        permissions: Permissions::new(),
    };
}

#[test]
fn build_object_test() {
    println!("Object Type: {:?}", "Not done");

    //println!("Object Type: {:?}", b.1);
    //assert_eq!(b.object_type, Businesses::LLC);
}
