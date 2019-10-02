use bitflags::*;
use std::fmt;
use std::mem;

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

    pub(crate) fn enable_essembly_permissions(&mut self) -> &Permissions<T> {
        //Permissions::<i32>::default()
        &self.mask.set(Flags::E_ACTIVE, true);
        &self.mask.set(Flags::I_ACTIVE, true);
        &self.mask.set(Flags::G_ACTIVE, true);
        &self.mask.set(Flags::O_ACTIVE, true);
        self
    }
}

bitflags! {
    pub(crate) struct Flags: u32 {
        const I = 0b00000000;
        const O = 0b00000000;
        const G = 0b00000000;
        const E = 0b00000000;
        const O_ACTIVE = 0b10000000;
        const E_ACTIVE = 0b10000000;
        const G_ACTIVE = 0b10000000;
        const I_ACTIVE = 0b10000000;
        const MIN_ACTIVE = Self::O_ACTIVE.bits | Self::E_ACTIVE.bits | Self::G_ACTIVE.bits | Self::I_ACTIVE.bits;
    }
}

pub(crate) fn isActive<T>(p: &Permissions<T>) -> bool {
    println!("Default bits: {:?}", p.mask.bits());
    println!("Default bits: {:?}", Flags::MIN_ACTIVE.bits());
    p.mask == Flags::MIN_ACTIVE
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
    assert_eq!(permissions.mask.bits(), 0b00000000);
}

#[test]
fn test_apply_essembly_mask() {
    let mut permissions = Permissions::<i32>::default();
    println!("Default bits: {:?}", permissions.mask.bits());
    assert_eq!(permissions.mask.bits(), 0b00000000);

    let permissions_with_essembly_mask = permissions.enable_essembly_permissions();
    println!(
        "Newly masked bits: {:?}",
        permissions_with_essembly_mask.mask.bits()
    );
    println!("Newly masked: {:?}", permissions_with_essembly_mask);

    assert_eq!(
        permissions.enable_essembly_permissions().mask,
        Flags::E_ACTIVE
    );

    assert_eq!(
        permissions.enable_essembly_permissions().mask,
        Flags::MIN_ACTIVE
    );
}

#[test]
//Default constructed permissions should be 0 (no acccess)
//for I (individual), O (organization), G (group), E (essembly)
fn other_tests() {
    let mut permissions = Permissions::<i32>::default();
    permissions.enable_essembly_permissions();
    assert_eq!(isActive(&permissions), true);
}

//Entity -> Permissions
//Given an individual, group, and org, provide access result to an entity
//
