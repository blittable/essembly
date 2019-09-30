#![doc(html_root_url = "https://docs.rs/essembly/0.1.0-alpha.1")]
#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub
)]
#![deny(intra_doc_link_resolution_failure)]
#![doc(test(
    no_crate_inject,
    attr(deny(warnings, rust_2018_idioms), allow(dead_code, unused_variables))
))]

//! Essembly Core.
//!
//! This module is the core of the Essembly libraries.  It provides
//! primitives that can be used to assemble other, secure and performant
//! libraries.
//!
//! It defines two core structs, [`Entity`] and [`User`].
//!
//! # Entity and User
//!
//! [`Entity`] and [`User`] are split on the following view:
//! Permissions (hashes, cryptographic keys, etc.) are *held*
//! by Users and *applied* to Entities.
//!
//! # Utility functions
//!
//! Utilities functions are provided for working with [`User`] /
//! [`Entity`] types. For example, creates an md5-based hash
//! using an organization identifier and a name.
//!
//! [`Entity`]: struct.Entity.html
//! [`User`]: struct.User.html
//! [`Id`]: trait.Id.html

#[cfg(feature = "accounts")]
pub mod accounts;
#[cfg(feature = "app")]
pub mod app;
#[cfg(feature = "cli")]
pub mod cli;
#[cfg(feature = "inventory")]
pub mod inventory;
#[cfg(feature = "pos")]
pub mod pos;
#[cfg(feature = "store")]
pub mod store;

use mockall::predicate::str;
use mockall::*;
use uuid::Uuid;
use uuid::{Variant, Version};

//Objects::Permissions::Users
//Such that Any Object can have any set of permissions, which can be applied to any number of users
pub enum PermissionLevel {
    SuperUser,
    PowerUser,
    StandardUser,
    LimitedUser,
    ReadOnlyUser,
}

type Permissions = PermissionLevel;

pub(crate) struct ExternalId {
    id: String,
}

pub(crate) struct InternalId {
    pub id: Uuid,
}

pub(crate) struct Identifier<T> {
    pub id_type: T,
}

pub(crate) enum Ids<T> {
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

///!The permissionable person in the system
struct User<T> {
    pub id: Ids<T>,
    pub permission_level: PermissionLevel,
}

impl<T> User<T> {
    fn set_permissions(&mut self, level: Permissions) {
        self.permission_level = level
    }
    fn get_permissions(&self) -> &PermissionLevel {
        &self.permission_level
    }
}

///!Linkable is an abstraction over a link between composables
trait Join<T>: FnOnce() -> T {}
impl<T, U> Join<T> for U where U: FnOnce() -> T {}

#[derive(Debug)]
enum Businesses {
    Listed,
    Private,
    LLC,
    Corportation,
    NonProfit,
    Subsidiary,
}
#[derive(Debug)]
enum Individuals {
    Independent,
    Dependent,
}
#[derive(Debug)]
enum Organizations {
    Social,
    Sport,
    Informal,
}

#[derive(Debug)]
pub(crate) enum EntityTypes {
    Business(Businesses),
    Individual(Individuals),
    Organization(Organizations),
}

type EntityType = EntityTypes;

#[derive(Debug)]
///!The entity against which permissions can be applied
struct Entity {
    entity_type: EntityType,
}

impl Entity {
    fn new(&self, _entity_type: EntityType) -> Self {
        return Self {
            entity_type: _entity_type,
        };
    }
}

///Core tests:
/// Construction of Entity
/// Construction of User
/// Construction of Permissions
/// Application of Permissions to User
/// Application of Permissions to Entity
/// Validate no leakage
/// Validate SHA1 entity map
/// Validate crytographic key usage
#[test]
fn get_permissions_test() {
    //We have for variants of IDs to test:
    //A) Internal - GUID
    //B) External - Any
    let ext_id = ExternalId {
        id: "John".to_string(),
    };
    let i = Identifier { id_type: ext_id };

    let j = User {
        id: Ids::Internal(i),
        permission_level: PermissionLevel::LimitedUser,
    };

    let guid_id = InternalId {
        id: Uuid::new_v5(&Uuid::NAMESPACE_X500, "foo".as_bytes()),
    };
    let k = Identifier { id_type: guid_id };

    let m = User {
        id: Ids::Internal(k),
        permission_level: PermissionLevel::LimitedUser,
    };
}

#[test]
fn build_entity_test() {
    let innertype = Businesses::LLC;
    let b = Entity {
        entity_type: EntityType::Business(innertype),
    };
    println!("Entity Type: {:?}", b);

    //println!("Entity Type: {:?}", b.1);
    //assert_eq!(b.entity_type, Businesses::LLC);
}
