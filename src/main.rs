fn main() {
    // Allocate our string
    let crab = fifteen_megs_of_crab();
    loop {
        // Make a copy of our string
        let larval = crab.clone();

        // No need to clean up the memory, it'll be fi--
        std::mem::forget(larval);
    }
}

/// Allocate a 15MB (ish) string comprised of the characters "CRAB"
fn fifteen_megs_of_crab() -> String {
    let mut big_ol_string = String::new();

    // a bit of back-of-the-envelope-maths
    // - the string "CRAB" is 4 bytes
    // - 15,000,000 / 4 == 3,750,000

    for _ in 0..3_750_000 {
        big_ol_string.push_str("CRAB");
    }

    big_ol_string
}
