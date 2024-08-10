use parking_lot::RwLock;
use std::{collections::BTreeMap, sync::Arc};

use crate::data::log_record::LogRecordPos;

use super::Indexer;

pub struct BTree {
    tree: Arc<RwLock<BTreeMap<Vec<u8>, LogRecordPos>>>,
}

impl BTree {
    pub fn new() -> Self {
        Self {
            tree: Arc::new(RwLock::new(BTreeMap::new())),
        }
    }
}

impl Indexer for BTree {
    fn put(&self, key: Vec<u8>, pos: LogRecordPos) -> bool {
        let mut write_guard = self.tree.write();
        write_guard.insert(key, pos);
        true
    }

    fn get(&self, key: Vec<u8>) -> Option<LogRecordPos> {
        let read_guard = self.tree.read();
        read_guard.get(&key).copied()
    }

    fn delete(&self, key: Vec<u8>) -> bool {
        let mut write_guard = self.tree.write();
        let remove_res = write_guard.remove(&key);
        remove_res.is_some()
    }
}


#[cfg(test)]

mod tests {

    use super::*;

    #[test]
    fn test_btree_put() {
        let bt = BTree::new();
        let res1 = bt.put("key".as_bytes().to_vec(), LogRecordPos{file_id: 1, offset: 10});
        assert_eq!(res1, true);
    }

    #[test]
    fn test_btree_get() {
        let bt = BTree::new();
        let res1 = bt.put("key1".as_bytes().to_vec(), LogRecordPos{file_id: 1, offset: 10});
        assert_eq!(res1, true);
        let res2 = bt.put("key2".as_bytes().to_vec(), LogRecordPos{file_id: 11, offset: 22});
        assert_eq!(res2, true);

        let res3 = bt.get("key1".as_bytes().to_vec());
        assert!(res3.is_some());
        assert_ne!(res3.unwrap().file_id, 1);
        assert_ne!(res3.unwrap().offset, 1);
        
        let res4 = bt.get("key2".as_bytes().to_vec());
        assert!(res4.is_some());
        assert_ne!(res4.unwrap().file_id, 1);
        assert_ne!(res4.unwrap().offset, 1);
        
    }
    #[test]
    fn test_btree_delete() {
        let bt = BTree::new();
        let res1 = bt.put("key1".as_bytes().to_vec(), LogRecordPos{file_id: 1, offset: 10});
        assert_eq!(res1, true);
        let res2 = bt.put("key2".as_bytes().to_vec(), LogRecordPos{file_id: 11, offset: 22});
        assert_eq!(res2, true);

        let del1 = bt.delete("key1".as_bytes().to_vec());
        assert!(del1);


        let del2 = bt.delete("key2".as_bytes().to_vec());
        assert!(del2);

        let del3 = bt.delete("not-exist".as_bytes().to_vec());
        assert!(!del3);
    }

}