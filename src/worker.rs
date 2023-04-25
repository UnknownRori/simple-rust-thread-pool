#[cfg(feature = "crossbeam")]
use crossbeam_channel::Receiver;

#[cfg(feature = "mpsc")]
use std::sync::mpsc::Receiver;

#[cfg(feature = "mpsc")]
use std::sync::{Arc, Mutex};

use std::thread::{self, JoinHandle};

use crate::message::Message;

#[derive(Debug)]
pub struct Worker {
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    /// Creates a new [`Worker`].
    ///
    /// ## Panic
    ///
    /// May panic when the OS cannot create thread
    #[cfg(feature = "crossbeam")]
    pub fn new(receiver: Receiver<Message>) -> Worker {
        let thread = thread::spawn(move || loop {
            if let Ok(message) = receiver.recv() {
                let _ = match message {
                    Message::NewJob(job) => job(),
                    Message::Terminate => break,
                };
            }
        });

        Worker {
            thread: Some(thread),
        }
    }

    #[cfg(feature = "mpsc")]
    pub fn new(receiver: Arc<Mutex<Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let _ = match receiver.lock().unwrap().recv().unwrap() {
                Message::NewJob(job) => job(),
                Message::Terminate => break,
            };
        });

        Worker {
            thread: Some(thread),
        }
    }

    /// Returns the take thread of this [`Worker`].
    pub fn take_thread(&mut self) -> Option<JoinHandle<()>> {
        self.thread.take()
    }
}
