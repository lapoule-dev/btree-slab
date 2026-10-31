//! The map on a slab grown by eighths, against the standard library's map:
//! random insertions and removals, then ranges and `retain`.
use btree_slab::BTreeMapByEighths;
use std::ops::Bound;

#[test]
fn a_map_on_a_slab_grown_by_eighths_matches_the_standard_map() {
	let mut x = 0x7f4a_7c15u32;
	let mut next = move || {
		x ^= x << 13;
		x ^= x >> 17;
		x ^= x << 5;
		x
	};
	for round in 0..2_000 {
		let mut a: BTreeMapByEighths<u32, u32> = Default::default();
		let mut o = std::collections::BTreeMap::new();
		let span = 1 + next() % 500;
		for step in 0..(next() % 600) {
			let k = next() % span;
			if next() % 3 == 0 {
				assert_eq!(a.remove(&k), o.remove(&k), "round {round}");
			} else {
				assert_eq!(a.insert(k, step), o.insert(k, step), "round {round}");
			}
		}
		assert!(a.iter().eq(o.iter()), "round {round}: entries");
		let lo = next() % (span + 5);
		let hi = lo + next() % 50;
		assert!(
			a.range((Bound::Excluded(lo), Bound::Included(hi)))
				.eq(o.range((Bound::Excluded(lo), Bound::Included(hi)))),
			"round {round}: range"
		);
		let d = 1 + next() % 5;
		a.retain(|k, _| k % d == 0);
		o.retain(|k, _| k % d == 0);
		assert!(a.iter().eq(o.iter()), "round {round}: retain");
	}
}
