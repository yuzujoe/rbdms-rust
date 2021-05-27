use anyhow::Result;

use crate::btree::Btree;
use crate::disk_manager::PageId;
use crate::buffer_pool::BufferPoolManager;
use crate::tuple;

pub struct SimpleTable {
    pub meta_page_id: PageId,
    pub num_key_elems: usize,
}

impl SimpleTable {
    pub fn create(&mut self, bufmgr: &mut BufferPoolManager) -> Result<(), E> {
        let btree = Btree::create(bufmgr)?;
        self.meta_page_id = btree.meta_page_id;
        ok(())
    }

    pub fn insert(&self, bufmgr: &mut BufferPoolManager, record: &[&[u8]]) -> Result<(), E> {
        let btree = Btree::new(self.meta_page_id);
        let mut key = vec![];
        tuple::encode(record[..self.num_key_elems].iter(), &mut key);
        let mut  value = vec![];
        tuple::encode(record[self.num_key_elems..].iter(), &mut value);
        btree.put(bufmgr, &key, &value)?;
        ok(())
    }
}

