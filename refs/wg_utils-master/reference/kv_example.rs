use kv::*;

#[derive(serde::Serialize, serde::Deserialize, PartialEq)]
struct SomeType {
    a: i32,
    b: i32
}

fn run() -> Result<(), Error> {
    // Configure the database
    let mut cfg = Config::new("./test/example");

    // Open the key/value store
    let store = Store::new(cfg)?;

    // A Bucket provides typed access to a section of the key/value store
    let test = store.bucket::<Raw, Raw>(Some("test"))?;

    // Set testing = 123
    test.set(b"test", b"123")?;
    assert!(test.get(b"test").unwrap().unwrap() == "123");
    assert!(test.get(b"something else").unwrap() == None);

    // Integer keys
    let aaa = store.bucket::<Integer, String>(Some("aaa"))?;
    aaa.set(1, "Testing");

    #[cfg(feature = "json-value")]
    {
        // Using a Json encoded type is easy, thanks to Serde
        let bucket = store.bucket::<&str, Json<SomeType>>(None)?;

        let x = SomeType {a: 1, b: 2};
        bucket.set("example", Json(x))?;

        let x: Json<SomeType> = bucket.get("example")?.unwrap();

        for item in bucket.iter() {
            let item = item?;
            let key: String = item.key()?;
            let value = item.value::<Json<SomeType>>()?;
            println!("key: {}, value: {}", key, value);
        }

        // A transaction
        bucket.transaction(|txn| {
            txn.set("x", Json(SomeType {a: 1, b: 2}))?;
            txn.set("y", Json(SomeType {a: 3, b: 4}))?;
            txn.set("z", Json(SomeType {a: 5, b: 6}))?;

            // A nested transaction
            test.transaction(|txn2| {
                let x = txn.get("x")?.unwrap();
                let v = format!("{}", x.as_ref().a);
                txn2.set(b"x", v.as_str())?;
                Ok(())
            })?;
            Ok(())
        })?;
    }
    Ok(())
}