use std::collections::HashMap;

const PADDING_BYTE: u8 = 61;

lazy_static! {
    static ref DECODING_MAP: HashMap<u8, u8> = {
        let mut decoding_map = HashMap::new();

        decoding_map.insert(65, 0);
        decoding_map.insert(66, 1);
        decoding_map.insert(67, 2);
        decoding_map.insert(68, 3);
        decoding_map.insert(69, 4);
        decoding_map.insert(70, 5);
        decoding_map.insert(71, 6);
        decoding_map.insert(72, 7);
        decoding_map.insert(73, 8);
        decoding_map.insert(74, 9);
        decoding_map.insert(75, 10);
        decoding_map.insert(76, 11);
        decoding_map.insert(77, 12);
        decoding_map.insert(78, 13);
        decoding_map.insert(79, 14);
        decoding_map.insert(80, 15);
        decoding_map.insert(81, 16);
        decoding_map.insert(82, 17);
        decoding_map.insert(83, 18);
        decoding_map.insert(84, 19);
        decoding_map.insert(85, 20);
        decoding_map.insert(86, 21);
        decoding_map.insert(87, 22);
        decoding_map.insert(88, 23);
        decoding_map.insert(89, 24);
        decoding_map.insert(90, 25);
        decoding_map.insert(97, 26);
        decoding_map.insert(98, 27);
        decoding_map.insert(99, 28);
        decoding_map.insert(100, 29);
        decoding_map.insert(101, 30);
        decoding_map.insert(102, 31);
        decoding_map.insert(103, 32);
        decoding_map.insert(104, 33);
        decoding_map.insert(105, 34);
        decoding_map.insert(106, 35);
        decoding_map.insert(107, 36);
        decoding_map.insert(108, 37);
        decoding_map.insert(109, 38);
        decoding_map.insert(110, 39);
        decoding_map.insert(111, 40);
        decoding_map.insert(112, 41);
        decoding_map.insert(113, 42);
        decoding_map.insert(114, 43);
        decoding_map.insert(115, 44);
        decoding_map.insert(116, 45);
        decoding_map.insert(117, 46);
        decoding_map.insert(118, 47);
        decoding_map.insert(119, 48);
        decoding_map.insert(120, 49);
        decoding_map.insert(121, 50);
        decoding_map.insert(122, 51);
        decoding_map.insert(48, 52);
        decoding_map.insert(49, 53);
        decoding_map.insert(50, 54);
        decoding_map.insert(51, 55);
        decoding_map.insert(52, 56);
        decoding_map.insert(53, 57);
        decoding_map.insert(54, 58);
        decoding_map.insert(55, 59);
        decoding_map.insert(56, 60);
        decoding_map.insert(57, 61);
        decoding_map.insert(43, 62);
        decoding_map.insert(47, 63);

        decoding_map
    };
}

pub struct Options {
    unpadded: bool,
}

const DEFAULT_OPTIONS: Options = Options { unpadded: false };

pub fn decode(encoded_text: String, options: Options) -> Option<Vec<u8>> {
    let mut plain_bytes: Vec<u8> = Vec::with_capacity(encoded_text.len());
    let mut byte_index: u8 = 0;
    let mut previous_byte: u8 = 0;

    for encoded_byte in encoded_text.as_bytes() {
        if *encoded_byte == PADDING_BYTE {
            return Some(plain_bytes);
        }

        let plain_result = DECODING_MAP.get(&encoded_byte);

        if let Some(&current_byte) = plain_result {
            let next_result = match byte_index {
                1 => Some(((previous_byte & 0b00111111) << 2) + ((current_byte & 0b00110000) >> 4)),
                2 => Some(((previous_byte & 0b00001111) << 4) + ((current_byte & 0b00111100) >> 2)),
                3 => Some(((previous_byte & 0b00000011) << 6) + (current_byte & 0b00111111)),
                _ => None,
            };

            if let Some(next_byte) = next_result {
                plain_bytes.push(next_byte);
            } else {
            }

            byte_index = if byte_index == 3 { 0 } else { byte_index + 1 };
            previous_byte = current_byte;
        } else {
            return None;
        }
    }

    Some(plain_bytes)
}

#[cfg(test)]
mod tests {
    use super::{decode as core_decode, DEFAULT_OPTIONS, Options};

    use base64::{decode, decode_config, encode, encode_config, STANDARD_NO_PAD};

    #[test]
    fn static_decode_examples_should_work() {
        assert_eq!(
            Some(string_to_bytes(String::from("Man"))),
            core_decode(String::from("TWFu"), DEFAULT_OPTIONS)
        );
        assert_eq!(
            Some(string_to_bytes(String::from("Ma"))),
            core_decode(String::from("TWE="), DEFAULT_OPTIONS)
        );
        assert_eq!(
            Some(string_to_bytes(String::from("M"))),
            core_decode(String::from("TQ=="), DEFAULT_OPTIONS)
        );
    }

    #[quickcheck]
    fn decode_should_work(text: String) -> bool {
        let encoded_text = encode(&text);

        decode(&encoded_text).ok() == core_decode(encoded_text, DEFAULT_OPTIONS)
    }

    #[quickcheck]
    fn decode_should_work_without_padding(text: String) -> bool {
        let encoded_text = encode_config(&text, STANDARD_NO_PAD);

        decode(&encoded_text).ok() == core_decode(encoded_text, Options { unpadded: true})
    }



    fn string_to_bytes(text : String) -> Vec<u8> {
        let mut vector = Vec::new();

        for v in text.as_bytes().iter() {
            vector.push(*v);
        }

        vector
    }
}
