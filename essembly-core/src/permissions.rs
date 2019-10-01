use bitflags::*;
use std::fmt;

/// Permissions for all entities
pub struct Permissions<T> {
    pub(crate) mask: Flags,
    pub(crate) schema: Option<T>,
    pub(crate) organization: Option<Organization>,
    pub(crate) group: Option<Group>,
}

impl<T> fmt::Debug for Permissions<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("Permissions")
            .field("mask", &self.mask)
            .field("group", &self.group)
            .field("organization", &self.organization)
            .finish()
    }
}

impl<T: Default> Default for Permissions<T> {
    fn default() -> Permissions<T> {
        Permissions::new(Default::default())
    }
}

impl<T> Permissions<T> {
    /// Creates a new futures-aware mutex.
    pub fn new(t: T) -> Permissions<T> {
        Permissions {
            mask: Default::default(),
            schema: None,
            organization: None,
            group: None,
        }
    }
}

bitflags! {
    pub(crate) struct Flags: u64 {
        const I = 0b00000000;
        const O = 0b00000000;
        const G = 0b00000000;
        const E = 0b00000000;
    }
}

// explicit `Default` implementation
impl Default for Flags {
    fn default() -> Flags {
        Flags::I | Flags::O | Flags::G | Flags::E
    }
}

#[derive(Debug)]
pub(crate) struct Organization {}

impl Organization {
    pub fn new() -> Organization {
        Organization {}
    }
}

#[derive(Debug)]
pub(crate) struct Group {}

impl Group {
    pub fn new() -> Group {
        Group {}
    }
}

/// Error raised by `permission`.
pub struct PermissionError {
    _p: (),
}

#[test]
//Default constructed permissions should be 0 (no acccess)
//for I (individual), O (organization), G (group), E (essembly)
fn test_create_permissions() {
    let permissions = Permissions::<i32>::default();
    //println!("Permissions: {:?}", permissions.mask);
    //println!("Permissions: {:?}", Flags::O);
    //println!("Permissions: {:?}", permissions.mask.bits());
    //println!("All: {:?}", Flags::all());
    //println!("All Bits: {:?}", Flags::all().bits());
    assert_eq!(permissions.mask.bits(), 0b00000000);
}

//Entity -> Permissions
//Given an individual, group, and org, provide access result to an entity
//
