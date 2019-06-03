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

    coefficients_58.reverse();

    let characters: Vec<char> = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz"
        .chars()
        .collect();

    coefficients_58
        .iter()
        .map(|value| value.to_usize().unwrap())
        .map(|coefficient| characters.get(coefficient).unwrap().to_string())
        .collect::<Vec<String>>()
        .join("")
}

pub fn decode(encoded_text: &String) -> Option<String> {
    let mut characters: Vec<u8> = ("123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz"
        .as_bytes())
    .into_iter()
    .map(|v| v.clone())
    .collect();

    let value_58_result =
        (*encoded_text.as_bytes())
            .into_iter()
            .try_fold(Zero::zero(), |acc: BigUint, ch| {
                characters
                    .iter()
                    .position(|v| v == ch)
                    .map(|value| acc * (58 as usize) + value)
            });

    value_58_result.and_then(|value_58| {
        let mut coefficients_256 = vec![];
        let mut value = value_58.clone();

        let divisor = BigUint::from(256 as usize);

        while !value.is_zero() {
            let remainder = &value % (256 as usize);
            coefficients_256.push(remainder.to_u8().unwrap());

            value = value.div_floor(&divisor);
        }

        coefficients_256.reverse();

        String::from_utf8(coefficients_256).ok()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use rust_base58::base58::{FromBase58, ToBase58};

    #[quickcheck]
    fn encode_should_work(text: String) -> bool {
        encode(&text) == *(&text.as_bytes()).to_base58()
    }

    #[quickcheck]
    fn encode_decode_should_work(text: String) -> bool {
        let encoded_text = (&text.as_bytes()).to_base58();
        let same_text = (&encoded_text.as_bytes())
            .from_base58()
            .ok()
            .and_then(|value| String::from_utf8(value).ok());

        dbg!(&text);
        dbg!(&encoded_text);
        dbg!(&same_text);
        dbg!(&encode(&text));
        dbg!(decode(&encode(&text)));
        decode(&encode(&text)) == same_text
    }

}
