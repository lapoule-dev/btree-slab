//! `retain` and `drain_filter` against the standard library's map, over
//! random trees deep enough to have internal nodes.
use btree_slab::BTreeMap;

#[test]
fn retain_and_drain_filter_match_the_standard_map() {
	let mut x = 0x2545_f491u32;
	let mut next = move || {
		x ^= x << 13;
		x ^= x >> 17;
		x ^= x << 5;
		x
	};
	for round in 0..20_000 {
		let mut a: BTreeMap<u32, u32> = BTreeMap::new();
		let mut o = std::collections::BTreeMap::new();
		let span = 1 + next() % 300;
		for _ in 0..(next() % 150) {
			let k = next() % span;
			a.insert(k, k);
			o.insert(k, k);
		}
		for _ in 0..(next() % 50) {
			let k = next() % span;
			a.remove(&k);
			o.remove(&k);
		}
		let d = 1 + next() % 6;
		if round % 2 == 0 {
			a.retain(|k, _| k % d == 0);
			o.retain(|k, _| k % d == 0);
		} else {
			let drained: Vec<(u32, u32)> = a.drain_filter(|k, _| k % d != 0).collect();
			let want: Vec<(u32, u32)> =
				o.iter().filter(|(k, _)| *k % d != 0).map(|(k, v)| (*k, *v)).collect();
			o.retain(|k, _| k % d == 0);
			assert_eq!(drained, want, "round {round}: drained");
		}
		assert_eq!(a.len(), o.len(), "round {round}: len");
		assert!(a.iter().eq(o.iter()), "round {round}: entries");
	}
}
