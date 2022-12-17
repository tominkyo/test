use ic_cdk_macros::*;
use ic_stable_memory::collections::vec::SVec;
use ic_stable_memory::{s, stable_memory_init, stable_memory_post_upgrade, stable_memory_pre_upgrade};
use crate::files::types::{MyStringsSlice, MyStrings};

#[query]
fn get_files(from: u64, to: u64) -> MyStringsSlice {
    let my_strings = s!(MyStrings);
    let mut result = MyStringsSlice::new();

    for i in from..to {
        let entry: String = my_strings.get_cloned(i).expect(format!("No entry at pos {}", i).as_str());
        result.push(entry);
    }

    result
}

