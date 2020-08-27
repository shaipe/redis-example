use redis::{Commands, ToRedisArgs};
use std::collections::{BTreeMap, BTreeSet, HashMap};

fn main() {
    println!("Hello, world!");

    println!("{:?}", do_something());
}
const REDIS_URL: &'static str = "redis://127.0.0.1:6379/4";
fn set_dic<T: ToRedisArgs + Clone>(map: BTreeMap<String, T>) {
    let client = redis::Client::open(REDIS_URL).unwrap();

    let conn = client.get_connection();
    if conn.is_err() {
        return;
    }
    let mut conn = conn.unwrap();
    // redis::pipe().set_multiple(map);
    for (key, val) in map.iter() {
        let v: () = redis::pipe()
            .cmd("SADD")
            .set(key, val.clone())
            .query(&mut conn)
            .unwrap_or(());
    }
}
fn do_something() -> redis::RedisResult<()> {
    let client = redis::Client::open(REDIS_URL).unwrap();
    let mut con = client.get_connection().unwrap();
    let mut v: BTreeMap<String, BTreeMap<u32, u32>> = BTreeMap::new();
    let mut dic: BTreeMap<u32, u32> = BTreeMap::new();
    dic.insert(8, 1);
    dic.insert(9, 1);
    dic.insert(10, 1);
    v.insert("18981772611".to_owned(), dic);
    set_dic(v);
    // con.set("key1", b"foo");
    // redis::cmd("SET").arg("my_key2").arg(b"testatatat").execute(&mut con);
    // for (key, val) in v.iter() {
    //     println!("redis cache={}", key);
    //     let v: () = redis::pipe()
    //         .cmd("SADD")
    //         .set(key, val.clone())
    //         .query(&mut con)
    //         .unwrap_or(());
    // }
    // 采用命令的方式也可以写入,测试成功

    // 采用con.sadd的方式也可以写入成功
    // let is_ok = con.sadd("ky5", v)?;
    // 获取写入的set值
    let x: BTreeMap<u32, u32> = con.smembers("18981772611")?;
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
