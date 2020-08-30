
use redis::{Commands, ToRedisArgs};
use std::thread;
use std::time::Duration as StdDuration;
fn main(){
    println!("{:?}", test_redis_pub());
    thread::sleep(StdDuration::new(100, 0));
}
//发布功能
fn test_redis_pub() -> redis::RedisResult<()>{
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut conn = client.get_connection()?;
    thread::spawn(move || {
        println!("channel");
        match redis::pipe().publish("channel_1","111").query(&mut conn) {
            Ok(v) => v,
            Err(_) => {}
        };
        println!("22");
        thread::sleep(StdDuration::new(100, 0));
    });
   
    Ok(())
}