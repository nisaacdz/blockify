#![cfg(test)]

#[test]
fn test1() {
    use blockify::trans::record::Record;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Record)]
    struct Detail<T> {
        val: T,
    }
}
