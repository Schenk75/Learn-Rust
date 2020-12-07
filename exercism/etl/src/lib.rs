use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut map: BTreeMap<char, i32> = BTreeMap::new();
    for (point, char_list) in h.iter() {
        for ch in char_list {
            let count = map.entry(ch.to_ascii_lowercase()).or_insert(0);
            *count += point;   
        }
    }

    map
}
