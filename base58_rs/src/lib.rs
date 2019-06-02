extern crate num;
extern crate num_bigint;
extern crate num_integer;
extern crate num_traits;

#[cfg(test)]
extern crate quickcheck;

#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

#[cfg(test)]
extern crate rust_base58;

use num_bigint::{BigUint, ToBigUint};
use num_integer::Integer;
use num_traits::{ToPrimitive, Zero};

pub fn encode(plain_text: &String) -> String {
    let value_256 =
        (*plain_text.as_bytes())
            .into_iter()
            .fold(Zero::zero(), |acc: BigUint, byte| {
                let value = byte.to_biguint().unwrap();

                acc * (256 as usize) + value
            });

    let mut coefficients_58 = vec![];
    let mut value = value_256.clone();

    let divisor = BigUint::from(58 as usize);

    while !value.is_zero() {
        coefficients_58.push(&value % (58 as usize));

        value = value.div_floor(&divisor);
    }

    let characters: Vec<String> = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz"
        .split("")
        .map(|char| char.to_string())
        .collect();

    coefficients_58
        .iter()
        .map(|value| value.to_usize().unwrap())
        .map(|coefficient| characters.get(coefficient).unwrap().to_string())
        .collect::<Vec<String>>()
        .join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    use rust_base58::base58::{FromBase58, ToBase58};

    #[quickcheck]
    fn encode_should_work(text: String) -> bool {
        println!("{}", encode(&"Cat".to_string()));
        encode(&text) == *(&text.as_bytes()).to_base58()
    }
}
