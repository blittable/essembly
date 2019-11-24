#![doc(html_root_url = "https://docs.rs/essembly/0.1.0-alpha.1")]
#![allow(
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

/*!
 Essembly Core

 # Brief overview

 This module contains the core primitives of Essembly and is used by all the other essembly-libraries.

 It primarily responsible for handling [`User`](struct.User.html) and [`Permissions`](struct.Permissions.html) but also supports locale and unit-of-measure
 libraries.

 # User, Object and Operation

 [`Object`](struct.Object.html),  [`User`](struct.User.html) and [`Operation`](struct.Operation.html) are divided on the following view:

 Permissions are attributes of Users and describe access to Entities
 and Operations.

 This creates the following permutations:
 User X has permissions to access data record Y (Object), but not permission to do operations Z.
 User X has permissions do operations Z, but no permissions to apply it to data value Y.

 and, of course,

 User X has permission to do operation Z *and* touch data record Y.

 Operation level permissions are required by Essembly, but Object level permissions are typically not required.
 A scenario where Object level permissions might be used: A financial-services employee may have access to most information
 about security transactions, but is restricted in the case of some (e.g. real-time updates)
*/

#![allow(dead_code)]
#![allow(unused_imports)]

#[macro_use]
extern crate bitflags;

pub mod entity;
pub mod error;
pub mod locale;
pub mod object;
pub mod permissions;
pub mod user;
