use std::num::Wrapping;

pub struct Identifier {
    pub id: String,
}

pub struct IdentifierGenerator {
    count: u32,
}

impl IdentifierGenerator {
    pub fn new() -> Self {
        Self {
            count: 0,
        }
    }
    pub fn generate<'a>(&'a mut self) -> Identifier {
        self.count += 1;
        let mut id = String::new();
        for i in 0..log2_32(self.count) + 1 {
            id.push(if self.count >> i & 0b1 == 0b1 {
                'l'
            } else {
                'I'
            });
        }
        Identifier { id }
    }
}

const TAB32: [u32; 32] = [
     0,  9,  1, 10, 13, 21,  2, 29,
    11, 14, 16, 18, 22, 25,  3, 30,
     8, 12, 20, 28, 15, 17, 24,  7,
    19, 27, 23,  6, 26,  5,  4, 31
];

fn log2_32(v: u32) -> u32 {
    let mut v = Wrapping(v);
    v |= v >> 1;
    v |= v >> 2;
    v |= v >> 4;
    v |= v >> 8;
    v |= v >> 16;
    v *= 0x07c4acdd;
    v >>= 27;
    TAB32[v.0 as usize]
}