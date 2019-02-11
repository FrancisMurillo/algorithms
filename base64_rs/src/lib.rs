#[macro_use]
extern crate lazy_static;

#[cfg(test)]
extern crate quickcheck;

#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

#[cfg(test)]
extern crate base64;

mod encode;

mod decode;
