use crate::crypto::digest::Digest;
use crate::crypto::sha2::Sha256;

pub fn randint(seed: &str) -> usize {
    let mut hasher = Sha256::new();
    hasher.input_str(seed);
    let mut bytes: [u8; 32] = [0; 32];
    hasher.result(&mut bytes);
    let mut result: usize = 0;
    for i in 0..8 {
        result = result << 8;
        result += bytes[i] as usize;
    }
    return result;
}

pub fn randint_range(seed: &str, min: usize, max: usize) -> usize {
    assert!(min <= max);
    let r = randint(seed);
    let diff = max - min;
    let r = r % (diff + 1);
    let r = r + min;
    return r;
}

pub fn shuffle<T>(seed: &str, elements: &mut Vec<T>) {
    let len = elements.len();
    if len <= 1 {
        return;
    }
    let max = len - 1;
    for i in 0..20 {
        let seed_a = format!("{}_{}_a", seed, i);
        let seed_b = format!("{}_{}_b", seed, i);
        let (a, b) = (
            randint_range(&seed_a, 0, max),
            randint_range(&seed_b, 0, max),
        );
        elements.swap(a, b);
    }
}
