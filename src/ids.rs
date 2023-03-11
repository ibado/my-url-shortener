const NUMBERS: &[char] = &['1', '2', '3', '4', '5', '6', '7', '8', '9'];
const LETTERS: &[char] = &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'];
const PADDING: u32 = 6666;

pub fn encode_id(i: u32) -> String {
    let i = i * PADDING;
    let mut result = String::new();
    let mut k = i;
    while k > 0 {
        let digit = k % 10;
        let chars = if digit % 2 == 0 { &NUMBERS } else { &LETTERS };
        result.push(chars[digit as usize]);
        k /= 10;
    }
    result
}

pub fn decode_id(encoded_id: &str) -> u32 {
    let initial = 10_usize.pow((encoded_id.len() - 1) as u32);
    let mut result = initial;
    let mut n = 0;
    for c in encoded_id.chars() {
        println!("char: {}", c);
        let d = NUMBERS
            .iter()
            .position(|num| *num == c)
            .or_else(|| LETTERS.iter().position(|letter| *letter == c))
            .unwrap();
        result += d * 10_usize.pow(n);
        n += 1;
    }
    ((result - initial) / (PADDING as usize)) as u32
}

#[cfg(test)]
mod test {
    use crate::ids::*;

    #[test]
    fn generate_different_values() {
        let id1 = 12345;
        let id2 = 12346;
        let id3 = 12349;
        assert_ne!(id1, id2);
        assert_ne!(id2, id3);
        assert_ne!(id3, id1);
    }

    #[test]
    fn round_trip_success() {
        let id = 34558;
        let e = encode_id(id);
        assert_eq!(id, decode_id(&e));
    }
}
