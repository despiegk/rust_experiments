fn main() {
    let db = sled::open("/tmp/welcome-to-sled").expect("open");

    // Batch insert
    const BATCH_SIZE: usize = 10_000;
    const TOTAL_ITEMS: usize = 1_000_000;
    let mut batch = sled::Batch::default();

    for i in 0..TOTAL_ITEMS {
        let key = format!("{:032x}", i); // Generates a 32-byte hexadecimal string
        let value = vec![b'x'; 50]; // Generates a 50-byte value
        batch.insert(key.as_bytes(), value);

        if i % BATCH_SIZE == 0 || i == TOTAL_ITEMS - 1 {
            db.apply_batch(batch).expect("apply batch");
            batch = sled::Batch::default(); // Reset the batch after applying
            println!("Inserted up to key {}", i);
        }
    }

    // Verify a subset of entries
    let test_indices = [0, 100, 10_000, 100_000, 999_999];
    for &i in &test_indices {
        let key = format!("{:032x}", i);
        let result = db.get(key).expect("get").expect("value not found");
        assert_eq!(result, vec![b'x'; 50]);
    }

    println!("Verified a subset of entries successfully.");

    // Optionally, verify the total number of items (this can be slow for large databases)
    let item_count: usize = db.iter().count();
    assert_eq!(item_count, TOTAL_ITEMS);

    println!("Total items: {}", item_count);

    // Ensure all operations are stable on disk before exiting
    db.flush().expect("flush");
}
