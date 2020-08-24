use redis::Commands;
use std::collections::{HashMap, HashSet, BTreeMap};

fn main() {
    println!("Hello, world!");

   println!("{:?}", do_something());
}

fn do_something() -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://:redis.root@127.0.0.1:6379/0").unwrap();
    
    let mut con = client.get_connection().unwrap();
    let mut v: BTreeMap<u32, u32> = BTreeMap::new();
    v.insert(2, 5);
    // con.set("key1", b"foo");
    // redis::cmd("SET").arg("my_key2").arg(b"testatatat").execute(&mut con);

    // let x1 = redis::pipe().cmd("HMSET").arg("key").arg(v).query(&mut con).unwrap();
    let is_ok = con.sadd("ky5", v)?;
    println!("{:?}", is_ok);
    let x: BTreeMap<u32, u32> = con.smembers("ky5")?;
    println!("{:?}", x);

    // println!("{:?}", con);
    // let count: i32 = con.get("my_counter")?;
    // let count = con.get("my_counter").unwrap_or(0i32);
    // let k: Option<String> = con.get("missing_key")?;
    // let name: String = con.get("my_name")?;
    // let bin: Vec<u8> = con.get("my_binary")?;
    // let map: HashMap<String, i32> = con.hgetall("my_hash")?;
    // let keys: Vec<String> = con.hkeys("my_hash")?;
    // let mems: HashSet<i32> = con.smembers("my_set")?;
    // let (k1, k2): (String, String) = con.get(&["k1", "k2"])?;
    Ok(())
}
