#![no_std]

use alloc::collections::btree_set::BTreeSet;
use pelite::pe64::{Pe, PeView};
extern crate alloc;
pub fn parble(buffer: &mut [u8], mut hide_import: impl FnMut(&mut [u8])) {
    let pe = PeView::from_bytes(buffer).unwrap();
    let mut to_hide = BTreeSet::new();
    let mut import = pe.imports().unwrap();
    for imp in import.iter() {
        let slice = pe.derva_slice_s(imp.image().OriginalFirstThunk, 0).unwrap();
        for va in slice.iter() {
            to_hide.insert(pe.rva_to_file_offset(*va).unwrap());
        }
    }
    for name in pe.exports().unwrap().names().unwrap() {
        to_hide.insert(pe.rva_to_file_offset(*name).unwrap());
    }
    for hide in to_hide {
        let offset = hide as usize;
        let mut zero_pos = offset;
        while zero_pos < buffer.len() && buffer[zero_pos] != 0 {
            zero_pos += 1;
        }
        let mut slice = &mut buffer[offset..zero_pos];
        hide_import(&mut slice);
    }
}
