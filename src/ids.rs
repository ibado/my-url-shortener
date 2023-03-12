const NUMBERS: &[char] = &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
const LETTERS: &[char] = &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'];
const PADDING: u32 = 61470;

pub fn encode_id(i: u32) -> String {
    let i = i + PADDING;
    let mut result = String::new();
    let mut k = i;
    let mut coin = false;
    while k > 0 {
        let digit = k % 10;
        let chars = if coin { &NUMBERS } else { &LETTERS };
        result.push(chars[digit as usize]);
        k /= 10;
        coin = !coin;
    }
    result
}

pub fn decode_id(encoded_id: &str) -> u32 {
    let initial = 10_usize.pow((encoded_id.len() - 1) as u32);
    let mut result = initial;
    let mut n = 0;
    for c in encoded_id.chars() {
        let d = NUMBERS
            .iter()
            .position(|num| *num == c)
            .or_else(|| LETTERS.iter().position(|letter| *letter == c))
            .unwrap();
        result += d * 10_usize.pow(n);
        n += 1;
    }
    ((result - initial) - (PADDING as usize)) as u32
}

#[cfg(test)]
mod test {
    use crate::ids::*;

    #[test]
    fn generate_different_values() {
        let seed = 1234;
        let mut ids = std::collections::HashSet::new();
        for i in 0..100_000 {
            ids.insert(encode_id(seed + i));
        }
        assert_eq!(100_000, ids.len());
    }

    #[test]
    fn round_trip_success() {
        let id = 34558;
        let e = encode_id(id);
        assert_eq!(id, decode_id(&e));
    }
}
