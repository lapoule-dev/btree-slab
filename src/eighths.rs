//! A slab that grows by an eighth of its room when full, not twice.
use cc_traits::{
	Capacity, Clear, Collection, CollectionMut, CollectionRef, Get, GetMut, Insert, Len, Remove,
	Reserve, SimpleCollectionMut, SimpleCollectionRef, WithCapacity,
};

/// A [`slab::Slab`] grown by an eighth of its room (at least four slots) when
/// it is full, instead of `Vec`'s doubling: a tree kept for a long time with
/// a steady number of nodes holds at most an eighth more room than nodes,
/// not up to twice as much.
#[derive(Clone, Debug)]
pub struct SlabByEighths<T>(slab::Slab<T>);

impl<T> Default for SlabByEighths<T> {
	fn default() -> Self {
		SlabByEighths(slab::Slab::new())
	}
}

impl<T> Collection for SlabByEighths<T> {
	type Item = T;
}

impl<T> CollectionRef for SlabByEighths<T> {
	type ItemRef<'a> = &'a T where Self: 'a;

	cc_traits::covariant_item_ref!();
}

impl<T> CollectionMut for SlabByEighths<T> {
	type ItemMut<'a> = &'a mut T where Self: 'a;

	cc_traits::covariant_item_mut!();
}

impl<T> SimpleCollectionRef for SlabByEighths<T> {
	cc_traits::simple_collection_ref!();
}

impl<T> SimpleCollectionMut for SlabByEighths<T> {
	cc_traits::simple_collection_mut!();
}

impl<T> WithCapacity for SlabByEighths<T> {
	fn with_capacity(capacity: usize) -> Self {
		SlabByEighths(slab::Slab::with_capacity(capacity))
	}
}

impl<T> Len for SlabByEighths<T> {
	fn len(&self) -> usize {
		self.0.len()
	}
}

impl<T> Capacity for SlabByEighths<T> {
	fn capacity(&self) -> usize {
		self.0.capacity()
	}
}

impl<T> Reserve for SlabByEighths<T> {
	fn reserve(&mut self, additional: usize) {
		self.0.reserve_exact(additional)
	}
}

impl<T> Get<usize> for SlabByEighths<T> {
	fn get(&self, key: usize) -> Option<&T> {
		self.0.get(key)
	}
}

impl<T> GetMut<usize> for SlabByEighths<T> {
	fn get_mut(&mut self, key: usize) -> Option<&mut T> {
		self.0.get_mut(key)
	}
}

impl<T> Insert for SlabByEighths<T> {
	type Output = usize;

	fn insert(&mut self, element: T) -> usize {
		// No vacant slot left: the next key is past the room.
		if self.0.vacant_key() >= self.0.capacity() {
			let room = self.0.capacity();
			self.0.reserve_exact((room / 8).max(4));
		}
		self.0.insert(element)
	}
}

impl<T> Remove<usize> for SlabByEighths<T> {
	fn remove(&mut self, key: usize) -> Option<T> {
		self.0.try_remove(key)
	}
}

impl<T> Clear for SlabByEighths<T> {
	fn clear(&mut self) {
		self.0.clear()
	}
}
