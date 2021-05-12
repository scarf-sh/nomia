#![no_std]
#![feature(allocator_api)]

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
