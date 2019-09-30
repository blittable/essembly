use uuid::{Uuid};
use uuid::{Version, Variant};
use mockall::predicate::str;
use mockall::*;

//Objects::Permissions::Users
//Such that Any Object can have any set of permissions, which can be applied to any number of users
pub enum PermissionLevel { SuperUser, PowerUser, StandardUser, LimitedUser, ReadOnlyUser } 

type Permissions = PermissionLevel;

pub(crate) struct ExternalId {
    id: String
}

pub(crate) struct InternalId {
    pub id: Uuid 
}

pub(crate) struct Identifier<T> {
    pub id_type: T
}

pub(crate) enum Ids<T> {
    Internal(Identifier<T>),
    External(Identifier<T>),
}

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

struct User<T> { 
    pub id: Ids<T>,
    pub permission_level: PermissionLevel
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
enum Businesses { Listed, Private, LLC, Corportation, NonProfit, Subsidiary }
#[derive(Debug)]
enum Individuals { Independent, Dependent }
#[derive(Debug)]
enum Organiations { Social, Sport, Informal }


#[derive(Debug)]
pub(crate) enum EntityTypes {
    Business(Businesses),
    Individuals(Individuals),
    Organiations(Organiations),
}

type EntityType = EntityTypes;

#[derive(Debug)]
struct Entity {
    entity_type: EntityType, 
}

impl Entity { 
    fn new(&self, _entity_type: EntityType) -> Self {
        return Self {
            entity_type: _entity_type 
        };
    }
}

#[test]
fn get_permissions_test() {

    //We have for variants of IDs to test:
    //A) Internal - GUID
    //B) External - Any
    let ext_id = ExternalId { id: "John".to_string() };
    let i = Identifier { id_type: ext_id };

    let j = User { id: Ids::Internal(i), permission_level: PermissionLevel::LimitedUser };

    let guid_id = InternalId { id: Uuid::new_v5(&Uuid::NAMESPACE_X500, "foo".as_bytes()) };
    let k = Identifier { id_type: guid_id };

    let m = User { id: Ids::Internal(k), permission_level: PermissionLevel::LimitedUser };
}


#[test]
fn build_entity_test() {

    let innertype = Businesses::LLC;
    let b = Entity { entity_type: EntityType::Business(innertype) };
    println!("Entity Type: {:?}", b);

    //println!("Entity Type: {:?}", b.1);
    //assert_eq!(b.entity_type, Businesses::LLC);


}





