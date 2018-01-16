// #[macro_use]
use super::*;
use std::collections::HashSet;
use std::thread;

fn some_vec() -> Vec<u8> {
    vec![1,3,5,6,7,10,12,17,18,19,26,33,35,
         36,39,40,50,52,54,54,56,57,60,63]
}



#[test]
fn to_hashset_and_back() {
    let start = vec![1,3,5,6,7,10,12,17,18,19,26,
                     33,35,36,39,40,50,52,54,54,
                     56,57,60,63];
    // LilBitSet can be constructed using collect()
    let lilbitset: LilBitSet = start.iter().collect();

    // conversions to and from HashSet<u8> are implemented
    let there: HashSet<u8> = lilbitset.into();
    let back_again: LilBitSet = there.into();

    assert_eq!(lilbitset, back_again);
}

#[test]
fn inserting_removing() {
    let mut v = vec![1,4,8,5,3,44,23];
    v.sort();
    v.dedup();

    let mut lilbitset = LilBitSet::new();
    for x in v.iter() {
        lilbitset.insert(*x);
    }
    let mut v2: Vec<u8> = lilbitset.into_iter().collect();
    v2.sort();

    assert_eq!(&v, &v2);
    for x in v.iter() {
        assert!(lilbitset.remove(*x));
    }

    v.clear();
    assert_eq!(&v, &lilbitset.into_iter().collect::<Vec<_>>());
}

#[test]
fn custom_macro() {
    let lilbitset1: LilBitSet = lilbits!{1,2,3,7,5,46,3,3,3};
    let lilbitset2: LilBitSet = vec![1,2,3,7,5,46,3,3,3].iter().collect();
    assert_eq!(lilbitset1, lilbitset2);
}


#[test]
fn super_fast() {
    let libset_const = vec![1,5,7].iter().collect();
    let mut lilbitset: LilBitSet = some_vec().iter().collect();
    for i in 0u32..1_000_000u32 {
        lilbitset = lilbitset.clone().union(libset_const);
        lilbitset.remove(((i+6) % 64) as u8);
        lilbitset.insert((i % 64) as u8);
    }
}


#[test]
#[should_panic]
fn boundaries() {
    // the golden rule! NO values > 63
    lilbits!{1,2,3,64};
}

#[test]
fn is_sync() {
    let lilbitset = lilbits!{1,2,5,3,55,9,5,62,33};

    let mut handles = vec![];
    for i in 0..20 {
        // let lilbitset_clone = lilbitset.clone();
        handles.push(thread::spawn(move || {
            lilbitset.contains(i);
        }));
    };
    for h in handles {
        let _ = h.join();
    }
}