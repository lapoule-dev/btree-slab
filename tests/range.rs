//! Ranges against the standard library's map: every bound kind, forwards
//! and backwards, over trees deep enough to have internal nodes, after
//! insertions and removals.
use btree_slab::BTreeMap;
use std::ops::Bound;

fn xorshift(mut x: u32) -> impl FnMut() -> u32 {
	move || {
		x ^= x << 13;
		x ^= x >> 17;
		x ^= x << 5;
		x
	}
}

#[test]
fn ranges_match_the_standard_map() {
	let mut next = xorshift(0x9e37_79b9);
	for round in 0..20_000 {
		let mut a: BTreeMap<u32, u32> = BTreeMap::new();
		let mut o = std::collections::BTreeMap::new();
		let span = 1 + next() % 200;
		for _ in 0..(next() % 120) {
			let k = next() % span;
			a.insert(k, k * 3);
			o.insert(k, k * 3);
		}
		for _ in 0..(next() % 60) {
			let k = next() % span;
			assert_eq!(a.remove(&k), o.remove(&k));
		}
		let bound = |next: &mut dyn FnMut() -> u32| match next() % 3 {
			0 => Bound::Included(next() % (span + 10)),
			1 => Bound::Excluded(next() % (span + 10)),
			_ => Bound::Unbounded,
		};
		let (lo, hi) = (bound(&mut next), bound(&mut next));
		let valid = match (lo, hi) {
			(Bound::Included(l), Bound::Included(h)) => l <= h,
			(Bound::Included(l) | Bound::Excluded(l), Bound::Included(h) | Bound::Excluded(h)) => {
				l < h
			}
			_ => true,
		};
		if !valid {
			continue;
		}
		let want: Vec<(u32, u32)> = o.range((lo, hi)).map(|(k, v)| (*k, *v)).collect();
		let got: Vec<(u32, u32)> = a.range((lo, hi)).map(|(k, v)| (*k, *v)).collect();
		assert_eq!(got, want, "round {round}: {lo:?}..{hi:?} forwards");
		let back: Vec<(u32, u32)> = a.range((lo, hi)).rev().map(|(k, v)| (*k, *v)).collect();
		let want_back: Vec<(u32, u32)> = want.iter().rev().copied().collect();
		assert_eq!(back, want_back, "round {round}: {lo:?}..{hi:?} backwards");
		for (_, v) in a.range_mut((lo, hi)) {
			*v += 1;
		}
		for (_, v) in o.range_mut((lo, hi)) {
			*v += 1;
		}
		assert!(a.iter().eq(o.iter()), "round {round}: range_mut");
	}
}
