use core::{
    marker::PhantomData,
    num::NonZeroU32,
};
use serde::Serialize;
use alloc::{
    collections::btree_map::{
        BTreeMap,
        Entry,
    },
};
use crate::TypeId;

pub type StringInterner = Interner<&'static str>;
pub type TypeIdInterner = Interner<TypeId>;

pub type StringSymbol<'a> = Symbol<'a, &'static str>;
pub type TypeIdSymbol<'a> = Symbol<'a, TypeId>;

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Symbol<'a, T>{
    id: NonZeroU32,
    marker: PhantomData<fn() -> &'a T>,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Interner<T> {
    #[serde(skip)]
    map: BTreeMap<T, usize>,
    vec: Vec<T>,
}

impl<T> Interner<T>
where
    T: Ord,
{
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
            vec: Vec::new(),
        }
    }
}

impl<T> Interner<T>
where
    T: Ord + Clone,
{
    pub fn intern_or_get(&mut self, s: T) -> Symbol<T> {
        let next_id = self.vec.len();
        let sym_id = match self.map.entry(s.clone()) {
            Entry::Vacant(vacant) => {
                vacant.insert(next_id);
                self.vec.push(s);
                next_id
            }
            Entry::Occupied(occupied) => {
                *occupied.get()
            }
        };
        Symbol {
            id: NonZeroU32::new((sym_id + 1) as u32).unwrap(),
            marker: PhantomData,
        }
    }

    pub fn resolve(&self, sym: Symbol<T>) -> Option<&T> {
        let idx = (sym.id.get() - 1) as usize;
        if idx >= self.vec.len() {
            return None
        }
        self.vec.get((sym.id.get() - 1) as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type StringInterner = Interner<&'static str>;

    fn assert_id(interner: &mut StringInterner, new_symbol: &'static str, expected_id: u32) {
        let actual_id = interner.intern_or_get(new_symbol).id.get();
        assert_eq!(
            actual_id,
            expected_id,
        );
    }

    fn assert_resolve<E>(interner: &mut StringInterner, symbol_id: u32, expected_str: E)
    where
        E: Into<Option<&'static str>>,
    {
        let actual_str = interner.resolve(Symbol { id: NonZeroU32::new(symbol_id).unwrap(), marker: PhantomData });
        assert_eq!(
            actual_str.cloned(),
            expected_str.into(),
        );
    }

    #[test]
    fn simple() {
        let mut interner = StringInterner::new();
        assert_id(&mut interner, "Hello", 1);
        assert_id(&mut interner, ", World!", 2);
        assert_id(&mut interner, "1 2 3", 3);
        assert_id(&mut interner, "Hello", 1);

        assert_resolve(&mut interner, 1, "Hello");
        assert_resolve(&mut interner, 2, ", World!");
        assert_resolve(&mut interner, 3, "1 2 3");
        assert_resolve(&mut interner, 4, None);
    }
}
