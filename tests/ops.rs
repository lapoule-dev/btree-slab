//! Random sequences of insertions, replacements, removals and lookups
//! against the standard library's map: the entries, in order, both ways,
//! after every operation.
use btree_slab::BTreeMap;

#[test]
fn random_operations_match_the_standard_map() {
	let mut x = 0x1234_5679u32;
	let mut next = move || {
		x ^= x << 13;
		x ^= x >> 17;
		x ^= x << 5;
		x
	};
	for round in 0..2_000 {
		let mut a: BTreeMap<u32, u32> = BTreeMap::new();
		let mut o = std::collections::BTreeMap::new();
		let span = 1 + next() % 500;
		for step in 0..(next() % 600) {
			let k = next() % span;
			match next() % 4 {
				0 | 1 => assert_eq!(a.insert(k, step), o.insert(k, step), "round {round}"),
				2 => assert_eq!(a.remove(&k), o.remove(&k), "round {round}"),
				_ => {
					if let (Some(v), Some(w)) = (a.get_mut(&k), o.get_mut(&k)) {
						*v += 1;
						*w += 1;
					}
				}
			}
			assert_eq!(a.get(&k), o.get(&k), "round {round}");
			assert_eq!(a.len(), o.len(), "round {round}");
		}
		assert!(a.iter().eq(o.iter()), "round {round}: forwards");
		assert!(a.iter().rev().eq(o.iter().rev()), "round {round}: backwards");
		assert!(a.keys().eq(o.keys()), "round {round}: keys");
		assert!(a.values().eq(o.values()), "round {round}: values");
	}
}
