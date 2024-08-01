use hashtable::HashTable;


#[test]
fn simple_test() {
    let mut phone_book = HashTable::<String, String>::new();
    for i in 0..11 {
        phone_book.insert(format!("{}", i), format!("{}", 1_000_000 + i));
    }

    phone_book.debug_dump();
    println!("----------------------------");
    phone_book.insert("hello".to_string(), "world".to_string());
    phone_book.debug_dump();
    println!("----------------------------");

    for i in 0..11 {
        let name = format!("{}", i);
        println!("{} -> {}", name, phone_book.get(&name).unwrap());
    }
}
