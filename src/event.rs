use std::io;
use std::sync::mpsc;
use std::thread;

use termion::event::Key;
use termion::input::TermRead;

pub enum Event<I> {
    Input(I),
}

pub struct Events {
    rx: mpsc::Receiver<Event<Key>>,
}

impl Events {
    pub fn new() -> Events {
        let (tx, rx) = mpsc::channel();
        let _input_handle = {
            let tx = tx.clone();
            info!("Spawning input thread");
            thread::spawn(move || {
                info!("Input thread spawned");
                let stdin = io::stdin();
                for evt in stdin.keys() {
                    info!("Event: {:?}", evt);
                    match evt {
                        Ok(key) => {
                            if let Err(e) = tx.send(Event::Input(key)) {
                                error!("Failed to send input event: {:?}", e);
                                return;
                            }
                        }
                        Err(e) => {
                            error!("Failed to read stdin keys, error: {:?}", e);
                        }
                    }
                }
                error!("Input thread aborted");
            })
        };
        Events { rx }
    }

    pub fn next(&self) -> Result<Event<Key>, mpsc::RecvError> {
        self.rx.recv()
    }
}
