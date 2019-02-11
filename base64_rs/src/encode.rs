use std::collections::HashMap;

pub type EncodedByte = u8;

const PADDING_BYTE: EncodedByte = 61;

lazy_static! {
    static ref ENCODING_MAP: HashMap<u8, EncodedByte> = {
        let mut encoding_map = HashMap::new();

        encoding_map.insert(0, 65);
        encoding_map.insert(1, 66);
        encoding_map.insert(2, 67);
        encoding_map.insert(3, 68);
        encoding_map.insert(4, 69);
        encoding_map.insert(5, 70);
        encoding_map.insert(6, 71);
        encoding_map.insert(7, 72);
        encoding_map.insert(8, 73);
        encoding_map.insert(9, 74);
        encoding_map.insert(10, 75);
        encoding_map.insert(11, 76);
        encoding_map.insert(12, 77);
        encoding_map.insert(13, 78);
        encoding_map.insert(14, 79);
        encoding_map.insert(15, 80);
        encoding_map.insert(16, 81);
        encoding_map.insert(17, 82);
        encoding_map.insert(18, 83);
        encoding_map.insert(19, 84);
        encoding_map.insert(20, 85);
        encoding_map.insert(21, 86);
        encoding_map.insert(22, 87);
        encoding_map.insert(23, 88);
        encoding_map.insert(24, 89);
        encoding_map.insert(25, 90);
        encoding_map.insert(26, 97);
        encoding_map.insert(27, 98);
        encoding_map.insert(28, 99);
        encoding_map.insert(29, 100);
        encoding_map.insert(30, 101);
        encoding_map.insert(31, 102);
        encoding_map.insert(32, 103);
        encoding_map.insert(33, 104);
        encoding_map.insert(34, 105);
        encoding_map.insert(35, 106);
        encoding_map.insert(36, 107);
        encoding_map.insert(37, 108);
        encoding_map.insert(38, 109);
        encoding_map.insert(39, 110);
        encoding_map.insert(40, 111);
        encoding_map.insert(41, 112);
        encoding_map.insert(42, 113);
        encoding_map.insert(43, 114);
        encoding_map.insert(44, 115);
        encoding_map.insert(45, 116);
        encoding_map.insert(46, 117);
        encoding_map.insert(47, 118);
        encoding_map.insert(48, 119);
        encoding_map.insert(49, 120);
        encoding_map.insert(50, 121);
        encoding_map.insert(51, 122);
        encoding_map.insert(52, 48);
        encoding_map.insert(53, 49);
        encoding_map.insert(54, 50);
        encoding_map.insert(55, 51);
        encoding_map.insert(56, 52);
        encoding_map.insert(57, 53);
        encoding_map.insert(58, 54);
        encoding_map.insert(59, 55);
        encoding_map.insert(60, 56);
        encoding_map.insert(61, 57);
        encoding_map.insert(62, 43);
        encoding_map.insert(63, 47);

        encoding_map
    };
}

pub struct Options {
    unpadded: bool,
}

trait Encodeable {
    fn encode(&self, options: Options) -> String;
}

impl Encodeable for String {
    fn encode(&self, options: Options) -> String {
        self.as_bytes().encode(options)
    }
}

const DEFAULT_OPTIONS: Options = Options { unpadded: false };

impl Encodeable for [u8] {
    fn encode(&self, options: Options) -> String {
        let encoded_bytes = EncodedStream::new(self, options).collect();

        String::from_utf8(encoded_bytes).unwrap()
    }
}

struct EncodedStream<T: AsRef<[u8]>> {
    index: usize,
    previous_byte: u8,
    byte_index: u8,
    padding: Option<u8>,
    unpadded: bool,
    bytes: T,
}

impl<T: AsRef<[u8]>> EncodedStream<T> {
    fn new(bytes: T, options: Options) -> Self {
        let Options { unpadded } = options;

        Self {
            index: 0,
            previous_byte: 0,
            byte_index: 0,
            padding: None,
            unpadded,
            bytes,
        }
    }
}

impl<T: AsRef<[u8]>> Iterator for EncodedStream<T> {
    type Item = EncodedByte;

    fn next(&mut self) -> Option<Self::Item> {
        let bytes = self.bytes.as_ref();
        let byte_index = self.byte_index;

        if self.index >= bytes.len() {
            if let Some(padding_remaining) = self.padding {
                if self.unpadded {
                    None
                } else if padding_remaining > 0 {
                    self.padding = Some(padding_remaining - 1);

                    Some(PADDING_BYTE)
                } else {
                    None
                }
            } else if byte_index == 0 {
                None
            } else if byte_index == 1 {
                self.padding = Some(2);

                let last_encoded = (self.previous_byte & 0b00000011) << 4;

                Some(*ENCODING_MAP.get(&last_encoded).unwrap())
            } else if byte_index == 2 {
                self.padding = Some(1);

                let last_encoded = (self.previous_byte & 0b00001111) << 2;

                Some(*ENCODING_MAP.get(&last_encoded).unwrap())
            } else {
                panic!("Invalid byte index")
            }
        } else {
            let current_byte = bytes[self.index];

            let next_encoded: EncodedByte = match byte_index {
                0 => (current_byte & 0b11111100) >> 2,
                1 => ((self.previous_byte & 0b00000011) << 4) + ((current_byte & 0b11110000) >> 4),
                2 => ((self.previous_byte & 0b00001111) << 2) + ((current_byte & 0b11000000) >> 6),
                3 => current_byte & 0b00111111,
                _ => panic!("Invalid byte index"),
            };

            self.byte_index = if byte_index == 3 { 0 } else { byte_index + 1 };
            self.index += if byte_index == 2 { 0 } else { 1 };
            self.previous_byte = current_byte;

            Some(*ENCODING_MAP.get(&next_encoded).unwrap())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use base64::{encode, encode_config, STANDARD_NO_PAD};

    #[test]
    fn static_encode_examples_should_work() {
        assert_eq!("TWFu", String::from("Man").encode(DEFAULT_OPTIONS));
        assert_eq!("TWE=", String::from("Ma").encode(DEFAULT_OPTIONS));
        assert_eq!("TQ==", String::from("M").encode(DEFAULT_OPTIONS));
        assert_eq!("", String::from("").encode(DEFAULT_OPTIONS));
    }

    #[quickcheck]
    fn encode_should_work(text: String) -> bool {
        encode(&text) == text.encode(DEFAULT_OPTIONS)
    }

    #[quickcheck]
    fn encode_should_work_without_padding(text: String) -> bool {
        encode_config(&text, STANDARD_NO_PAD) == text.encode(Options { unpadded: true })
    }
}
