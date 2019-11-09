#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

//! A queue of event elements.
//!
//! See [`EventQueue`] for more details.
//!
//! [`EventQueue`]: struct.EventQueue.html
use futures_core::ready;
use std::collections::HashMap;
use std::env;
use std::io;
use std::task::{Context, Poll};
use std::time::Duration;
use std::{thread, time};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::timer::{delay_queue, Delay, DelayQueue, Error};

type Value = String;
type CacheKey = String;

/// A queue of delayed events.
///
/// Once an element is inserted into the `EventQueue`, it is yielded once the
/// specified deadline has been reached.
#[derive(Debug)]
pub struct EventQue {
    pub entries: HashMap<CacheKey, (Value, delay_queue::Key)>,
    pub expirations: DelayQueue<CacheKey>,
}

impl EventQue {
    pub fn new() -> EventQue {
        EventQue {
            entries: HashMap::<CacheKey, (Value, delay_queue::Key)>::new(),
            expirations: DelayQueue::<CacheKey>::new(),
        }
    }

    fn insert(&mut self, key: CacheKey, value: Value, future_time: u64) {
        let delay = self
            .expirations
            .insert(key.clone(), Duration::from_secs(future_time));

        self.entries.insert(key, (value, delay));
    }

    fn get(&self, key: &CacheKey) -> Option<&Value> {
        self.entries.get(key).map(|&(ref v, _)| v)
    }

    fn remove(&mut self, key: &CacheKey) {
        if let Some((_, cache_key)) = self.entries.remove(key) {
            self.expirations.remove(&cache_key);
        }
    }

    fn poll_purge(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Error>> {
        while let Some(res) = ready!(self.expirations.poll_next(cx)) {
            let entry = res?;
            self.entries.remove(entry.get_ref());
        }

        Poll::Ready(Ok(()))
    }

    fn poll_peek(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Error>> {
        while let Some(res) = ready!(self.expirations.poll_next(cx)) {
            let entry = res?;
            println!(" entry = {:?}", entry);
            self.entries.get(entry.get_ref());
        }

        Poll::Ready(Ok(()))
    }
}

fn main() {}

#[tokio::main]
async fn event_que_immediate_delay() {
    use tokio::timer::delay;
    use tokio::timer::timer::Handle;
    use tokio_test::task;
    use tokio_test::{assert_ok, assert_pending, assert_ready, clock};

    let mut queue = EventQue::new();
    let key = "foo".to_string();
    queue.insert("foo".to_string(), "eat_chicken".to_string(), 10);

    let queried_key = queue.get(&key);
    println!(" key = {:?}", queried_key);

    let ten_millis = time::Duration::from_secs(3);
    let now = time::Instant::now();
    thread::sleep(ten_millis);

    let when = tokio::clock::now() + Duration::from_millis(100);

    println!(" 2 key = {:?}", queried_key);

    queue.insert("foog".to_string(), "eat_chicken".to_string(), 10);
    let ten_millis = time::Duration::from_secs(3);

    tokio::spawn(async move {
        let mut queue = EventQue::new();
        let key = "food".to_string();
        queue.insert(key, "eat_chicken".to_string(), 1);

        let ten_millis = time::Duration::from_secs(3);
        thread::sleep(ten_millis);

        let key = "food".to_string();
        let q3 = queue.get(&key);
        println!(" async key = {:?}", q3);

        loop {
            let ten_millis = time::Duration::from_secs(1);
            thread::sleep(ten_millis);
        }
    });

    let ten_millis = time::Duration::from_secs(3);
    let now = time::Instant::now();
    thread::sleep(ten_millis);
}
