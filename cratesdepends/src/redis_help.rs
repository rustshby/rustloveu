//! redis_help uses redis crate
//!
//!
//!

extern crate redis;
use redis::{Client, Commands};

pub fn fetch_an_integer() -> redis::RedisResult<isize> {
    // connect to redis
    let client = Client::open("redis://127.0.0.1:6379")?;
    let mut con = client.get_connection()?;
    // throw away the result, just make sure it does not fail
    let _: () = con.set("my_key", 42)?;
    // read back the key and return it.  Because the return value
    // from the function is a result for integer this will automatically
    // convert into one.
    con.get("my_key")
}
