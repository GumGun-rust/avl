//pub mod level;
pub mod iter_ref;
pub mod iter_ref_mut;
pub mod into_iter;

#[cfg(feature = "unchecked_mut")]
pub mod iter_ref_mut_unchecked;

#[cfg(feature = "into_precompiled")]
pub mod into_iter_precompiled;

use super::{
    structs::{
        Side,
    },
    Map,
    MapNode,
    MapLink,
};

use std::marker::PhantomData;


pub struct IterRef<'a, KeyType:Ord, ContentType> (
    IterRefEnum<'a, KeyType, ContentType>
);

enum IterRefEnum<'a, KeyType:Ord, ContentType> {
    NewIter(&'a Map<KeyType, ContentType>),
    Iter{
        current: MapLink<KeyType, ContentType>,
        phantom0: PhantomData<&'a mut KeyType>,
        phantom1: PhantomData<&'a mut ContentType>,
    }
}

pub struct IterRefMut<'a, KeyType:Ord, ContentType> (
    IterRefMutEnum<'a, KeyType, ContentType>
);

enum IterRefMutEnum<'a, KeyType:Ord, ContentType> {
    NewIter(&'a mut Map<KeyType, ContentType>),
    Iter{
        current: MapLink<KeyType, ContentType>,
        phantom0: PhantomData<&'a mut KeyType>,
        phantom1: PhantomData<&'a mut ContentType>,
    }
}

pub struct IntoIter<KeyType:Ord, ContentType> {
    map: Map<KeyType, ContentType>,
    iter_data: IntoIterEnum<KeyType, ContentType>
}

pub(crate) struct EmptyIter<'a, KeyType:Ord, ContentType> {
    map: &'a mut Map<KeyType, ContentType>,
    iter_data: IntoIterEnum<KeyType, ContentType>
}

enum IntoIterEnum<KeyType:Ord, ContentType> {
    NewIter,
    Iter{
        next: Option<MapLink<KeyType, ContentType>>,
        phantom0: PhantomData<KeyType>,
        phantom1: PhantomData<ContentType>,
    },
    
}

impl<KeyType:Ord, ContentType> Map<KeyType, ContentType> {
    
    fn next_node(current:MapLink<KeyType, ContentType>) -> Option<MapLink<KeyType, ContentType>> {
        let current_ref = unsafe{current.as_ref()};
        if let Some(mut pivot) = current_ref.son[Side::Right] {
            loop {
                let pivot_ref = unsafe{pivot.as_ref()};
                match pivot_ref.son[Side::Left] {
                    Some(son) => {
                        pivot = son;
                    },
                    None => {
                        return Some(pivot);
                    }
                }
            }
        }
        let mut pivot = current;
        loop {
            let side = MapNode::get_side(pivot);
            match side {
                Some(Side::Left) => {
                    return unsafe{pivot.as_ref()}.father;
                },
                Some(Side::Right) => {
                    pivot = unsafe{pivot.as_ref()}.father.expect("should have father");
                },
                None => {
                    return None;
                }
            }
        }
    }
    
}

