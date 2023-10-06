/// one thread owner with one worker model
/// worker maintain Arc<DB>
/// owner
///     send payload
///     maintain Arc<DB> instance
/// worker thread
///     recv payload
///     read/write Arc<DB> instance
///
use crossbeam_channel::{bounded, Receiver, Sender};
use std::{
    sync::atomic::{AtomicBool, Ordering},
    sync::{Arc, RwLock},
    thread::{Builder, JoinHandle},
    time::Duration,
};

#[derive(Debug)]
struct Payload {
    serial_num: usize,
    action: DBAction,
}

struct DB {
    counter: usize,
}

impl DB {
    fn new() -> Self {
        Self { counter: 0 }
    }
    fn read(&self) -> usize {
        self.counter
    }
    fn increase(&mut self, counter: usize) -> usize {
        self.counter += counter;
        self.counter
    }
}

#[derive(Debug)]
enum DBAction {
    Read,
    Increase(usize),
}

struct Owner {
    exit_worker: Arc<AtomicBool>,
    worker: Option<JoinHandle<()>>,
    sender: Sender<Payload>,
    db: Arc<RwLock<DB>>,
}

trait NotifyPayload {
    fn notify_payload(&self, payload: Payload);
}

impl Owner {
    fn new() -> Self {
        let (sender, receiver) = bounded(100);
        let db = Arc::new(RwLock::new(DB::new()));
        let exit_worker = Arc::new(AtomicBool::new(false));
        let exit_clone = exit_worker.clone();
        let db_clone = db.clone();
        let worker = Builder::new()
            .name(String::from("worker thread"))
            .spawn(move || {
                let worker = Worker::new();
                worker.do_work(receiver, exit_clone, db_clone);
            })
            .unwrap();

        Self {
            exit_worker,
            worker: Some(worker),
            sender,
            db,
        }
    }
    fn join(&mut self) {
        self.exit_worker.store(true, Ordering::Relaxed);
        if let Some(handle) = self.worker.take() {
            let result = handle.join();
            if result.is_err() {
                dbg!("handle stop err");
            }
        }
    }
    fn get_counter(&self) -> usize {
        self.db.read().unwrap().read()
    }
}

impl NotifyPayload for Owner {
    fn notify_payload(&self, payload: Payload) {
        dbg!(self.sender.len());
        if let Err(e) = self.sender.send(payload) {
            dbg!(e);
        }
    }
}

struct Worker;

impl Worker {
    fn new() -> Self {
        Worker
    }
    fn do_work(
        &self,
        receiver: Receiver<Payload>,
        exit_worker: Arc<AtomicBool>,
        db: Arc<RwLock<DB>>,
    ) {
        while !exit_worker.load(Ordering::Relaxed) {
            let payload = receiver.recv_timeout(Duration::from_millis(500));
            dbg!(&payload);

            if let Ok(Payload { serial_num, action }) = payload {
                dbg!(serial_num);
                match action {
                    DBAction::Read => {
                        let counter = db.read().unwrap().read();
                        dbg!(counter);
                    }
                    DBAction::Increase(counter) => {
                        let counter = db.write().unwrap().increase(counter);
                        dbg!(counter);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Owner;
    use crate::utils::single_thread::{DBAction, NotifyPayload, Payload};
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn test_notify() {
        let mut owner = Owner::new();

        // send payload
        owner.notify_payload(Payload {
            serial_num: 1,
            action: DBAction::Read,
        });
        owner.notify_payload(Payload {
            serial_num: 2,
            action: DBAction::Increase(10),
        });
        owner.notify_payload(Payload {
            serial_num: 3,
            action: DBAction::Read,
        });

        sleep(Duration::from_secs(2));

        let counter = owner.get_counter();
        assert_eq!(10, counter);

        owner.join();
    }
}
