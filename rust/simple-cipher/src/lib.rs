pub fn encode(key: &str, s: &str) -> Option<String> {
    // todo!("Use {key} to encode {s} using shift cipher")
    if !is_key_valid(key) {
        return None;
    }

    let ss: Vec<u8> = s
        .as_bytes()
        .iter()
        .enumerate()
        .map(|(idx, ch)| {
            let st = key.as_bytes().get(idx % key.len()).unwrap();

            let st_normal = st - b'a';
            let ch_normal = ch - b'a';

            b'a' + (ch_normal + st_normal) % 26
        })
        .collect();

    String::from_utf8(ss).ok()
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    // todo!("Use {key} to decode {s} using shift cipher")
    if !is_key_valid(key) {
        return None;
    }

    let ss: Vec<u8> = s
        .as_bytes()
        .iter()
        .enumerate()
        .map(|(idx, ch)| {
            let st = key.as_bytes().get(idx % key.len()).unwrap();

            let st_normal = st - b'a';
            let ch_normal = ch - b'a';

            b'a' + (ch_normal + 26 - st_normal) % 26
        })
        .collect();

    String::from_utf8(ss).ok()
}

pub fn encode_random(s: &str) -> (String, String) {
    // todo!("Generate random key with only a-z chars and encode {s}. Return tuple (key, encoded s)")
    let key_len = rand::random_range(100..=1000);
    let mut vec = Vec::with_capacity(key_len);

    for _ in 0..key_len {
        vec.push(b'a' + rand::random_range(0..26));
    }

    let key = String::from_utf8(vec).unwrap();
    let encoded = encode(&key, s).unwrap();

    (key, encoded)
}

fn is_key_valid(key: &str) -> bool {
    !key.is_empty()
        && key
            .chars()
            .all(|ch| ch.is_alphabetic() && ch.is_lowercase())
}
