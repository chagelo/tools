use std::fmt::Debug;

pub trait Hashable {
    fn hash(&self) -> usize;
}

impl Hashable for String {
    fn hash(&self) -> usize {
        let mut result: usize = 5381;
        for c in self.bytes() {
            result = (result << 5).wrapping_add(result).wrapping_add(c as usize);
        }
        result
    }
}

impl Hashable for usize{
    fn hash(&self) -> usize {
        *self   
    }
}

#[derive(Clone, Default)]
struct HashCell<Key, Value> {
    key: Key,
    value: Value,
    taken: bool,
}

pub struct HashTable<Key, Value> {
    cells: Vec<HashCell<Key, Value>>,
    taken_count: usize,
}

impl<Key: Clone + Default + Hashable + Debug + PartialEq, Value: Clone + Default + Debug>
    HashTable<Key, Value>
{
    pub fn new() -> Self {
        const INITIAL_CAPCITY: usize = 3;
        Self {
            cells: vec![HashCell::<_, _>::default(); INITIAL_CAPCITY],
            taken_count: 0,
        }
    }
    pub fn debug_dump(&self) {
        for cell in self.cells.iter() {
            if cell.taken {
                println!("{:?} -> {:?}", cell.key, cell.value)
            } else {
                println!("x");
            }
        }
    }

    fn extend(&mut self) {
        assert!(self.cells.len() > 0);
        let mut new_self = Self {
            cells: vec![HashCell::<_, _>::default(); self.cells.len() * 2],
            taken_count: 0
        };

        for cell in self.cells.iter() {
            if cell.taken {
                new_self.insert(cell.key.clone(), cell.value.clone()); 
            }
        }
        *self = new_self;
    }

    pub fn insert(&mut self, key: Key, value: Value) {
        if let Some(old_value) = self.get_mut(&key) {
            *old_value = value;
        } else {
            if self.taken_count >= self.cells.len() {
                self.extend();
            }
    
            assert!(self.taken_count < self.cells.len());

            let mut index = key.hash() % self.cells.len();

            while self.cells[index].taken {
                index = (index + 1) % self.cells.len();
            }
            self.cells[index].taken = true;
            self.cells[index].key = key;
            self.cells[index].value = value;
            self.taken_count += 1;
        }
    }

    pub fn get(&self, key: &Key) -> Option<&Value> {
        self.get_index(key).map(|index|&self.cells[index].value) 
    }

    pub fn get_index(&self, key: &Key) -> Option<usize> {
        let mut index = key.hash() % self.cells.len();
        
        for _ in 0..self.cells.len(){
            if !self.cells[index].taken {
                break;
            }
            
            if self.cells[index].key == *key {
                break;
            }

            index = (index + 1) % self.cells.len();
        }

        if self.cells[index].taken && self.cells[index].key == *key {
            Some(index)
        }else {
            None
         }
    }

    pub fn get_mut(&mut self, key: &Key) -> Option<&mut Value> {
        self.get_index(key).map(|index|&mut self.cells[index].value) 
    }
}