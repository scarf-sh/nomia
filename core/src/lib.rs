#![no_std]
#![cfg_attr(feature = "alloc", feature(allocator_api))]
#![cfg_attr(feature = "alloc", feature(try_reserve))]
#![cfg_attr(feature = "alloc", feature(arbitrary_self_types))]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod name;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
