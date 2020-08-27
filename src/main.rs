use redis::{Commands, ToRedisArgs};
use std::collections::{BTreeMap, BTreeSet};

fn main() {
    println!("Hello, world!");

    println!("{:?}", do_something());
}
fn set_hashs<K: AsRef<str>, T: ToRedisArgs + Copy>(map: BTreeMap<K, BTreeMap<T, T>>) {
    let client = redis::Client::open("redis://127.0.0.1:6379/0").unwrap();
    let conn = client.get_connection();
    if conn.is_err() {
        return;
    }
    let mut conn = conn.unwrap();
    // redis::pipe().set_multiple(map);
    println!("{}", map.len());
    for (key, h) in map.iter() {
        for (f, val) in h {
            match redis::pipe()
                .cmd("HSET")
                .arg(key.as_ref())
                .arg(f.clone())
                .arg(val.clone())
                .query(&mut conn)
            {
                Ok(v) => v,
                Err(_) => {}
            }
        }
    }
}
fn do_something() -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1:6379/0").unwrap();
    let mut con = client.get_connection().unwrap();
    let mut v: BTreeMap<String, BTreeMap<u32, u32>> = BTreeMap::new();
    let mut dic: BTreeMap<u32, u32> = BTreeMap::new();
    dic.insert(8, 1);
    dic.insert(9, 1);
    dic.insert(10, 1);
    v.insert("18981772611".to_owned(), dic);
    set_hashs(v);
    // con.set("key1", b"foo");
    // redis::cmd("SET").arg("my_key2").arg(b"testatatat").execute(&mut con);
    // for (key, h) in v.iter() {
    //     println!("redis cache={}", key);
    //     for (f, val) in h {
    //         let v: () = redis::pipe()
    //             .cmd("HSET")
    //             .arg(key)
    //             .arg(*f)
    //             .arg(*val)
    //             // .hset("store", key, val.clone())
    //             .query(&mut con)
    //             .unwrap();
    //     }
    // }
    // 采用命令的方式也可以写入,测试成功

    // 采用con.sadd的方式也可以写入成功
    // let is_ok = con.sadd("ky5", v)?;
    println!("{:?}", 2);

    // 获取写入的set值
    let x: BTreeMap<u32, u32> = con.hgetall("18981772611")?;
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
