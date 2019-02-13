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

pub fn decode(encoded_text: &String) -> Option<Vec<u8>> {
    let mut plain_bytes: Vec<u8> = Vec::with_capacity(encoded_text.len());
    let mut byte_index: u8 = 0;
    let mut previous_byte: u8 = 0;

    for encoded_byte in encoded_text.as_bytes() {
        let plain_result = DECODING_MAP.get(&encoded_byte);

        if plain_result == None {
            return None;
        }

        let Some(&current_byte) = plain_result;

        match byte_index {
            0 => {}
            1 => ((previous_byte & 0b00111111) << 2) + (current_byte & 0b00),
            2 => {}
            3 => {}
            _ => panic!("Invalid byte index"),
        }

        byte_index = if byte_index == 3 { 0 } else { byte_index + 1 };
        previous_byte = current_byte;
    }

    Some(plain_bytes)
}
