// use std::iter;

// pub(crate) const sys: &'static str = "sys";
// pub(crate) const org: &'static str = "org";
// pub(crate) const group: &'static str = "group";
// pub(crate) const user: &'static str = "user";

// #[derive(Debug, Copy, Clone)]
// struct Permissions {
//     sys: u8,
//     org: u8,
//     group: u8,
//     user: u8,
// }

// impl Permissions {
//     //new() returns with all values unitialized
//     pub fn new() -> Self {

//         Permissions {
//             sys: 0b00000000,
//             org: 0b00000000,
//             group: 0b00000000,
//             user: 0b00000000,
//         }
//     }

//     fn apply(&mut self) -> impl Iterator<Item = u8> {
//         let s = iter::once(self.sys);
//         let o = iter::once(self.org);
//         let g = iter::once(self.group);
//         let u = iter::once(self.user);
//         s.chain(o).chain(g).chain(u)
//     }

//     //by convention `active` is iff sys, org, group, and user have
//     //lead bit as 1.  All users need to be active in all groups in
//     //order to use the system
//     pub fn is_active(&self) -> bool {

//         let result = (self.sys << 7) & (self.org << 7) & (self.group << 7) & (self.user << 7);
//         result == 128_u8
//     }

//     //Set the minimum permissions usage
//     pub fn activate(&mut self) {
//         self.sys = 1;
//         self.org = 1;
//         self.group = 1;
//         self.user = 1;
//     }
// }

// fn debug_permissions(p: &Permissions) {
//     println!("sys: {:?}", &p.sys);
//     println!("org: {:?}", &p.org);
//     println!("group: {:?}", &p.group);
//     println!("user: {:?}", &p.user);
// }

// #[test]
// //Refresher course - leave the println!s
// fn raw_test() {
//     let n: u8 = 0b00010001;
//     let m: u8 = 0b00000101;
//     let p: u8 = 0b00010000;
//     let w: u8 = 0b00010001;

//     // println!("M is: {:?}", &m);
//     // println!("N is: {:?}", &n);
//     // println!("P is: {:?}", &p);
//     // println!("W is: {:?}", &w);
//     // println!("M n bytes is: {:b}", &m);
//     // println!("N n bytes is: {:b}", &n);
//     // println!("P n bytes is: {:b}", &p);
//     // println!("W n bytes is: {:b}", &w);

//     let r = (n << 7) & (m << 7) & (p << 7) & (w << 7);

//     //println!("R in bytes is: {:b}", &r);
// }

// #[test]
// fn new_permissions_is_not_active() {

//     let mask = Permissions::new();

//     assert_eq!(mask.is_active(), false);
// }

// #[test]
// fn activate_all() {
//     let mut mask = Permissions::new();
//     mask.activate();

//     assert_eq!(mask.is_active(), true);
// }

// #[test]
// fn org_only_not_active() {
//     let mut mask = Permissions::new();
//     mask.org = 0b00000001;

//     assert_eq!(mask.is_active(), false);
// }

// #[test]
// fn all_but_org_not_active() {
//     let mut mask = Permissions::new();
//     mask.activate();
//     mask.org = 0b00000000;

//     assert_eq!(mask.is_active(), false);
// }

// #[test]
// fn little_endian_bits_only_not_active() {
//     let mut mask = Permissions::new();
//     mask.activate();
//     mask.org = 0b10001000;
//     mask.sys = 0b10001000;
//     mask.group = 0b10100000;
//     mask.user = 0b10001000;

//     assert_eq!(mask.is_active(), false);
// }

// #[test]
// fn mixed_permissions_is_active() {
//     let mask = &mut Permissions::new();
//     mask.activate();
//     mask.org = 0b00001001;
//     mask.sys = 0b10001001;
//     mask.group = 0b00100001;
//     mask.user = 0b10001001;

//     assert_eq!(mask.is_active(), true);
// }
