use super::log::DataDogLog;
use crate::client::DataDogClient;
use flume::{Receiver, Sender, TryRecvError};

pub fn logger_thread<T: DataDogClient>(
    mut client: T,
    logs: Receiver<DataDogLog>,
    mut selflog: Option<Sender<String>>,
) {
    let mut store: Vec<DataDogLog> = Vec::new();

    loop {
        match logs.try_recv() {
            Ok(msg) => {
                if store.len() < 50 {
                    store.push(msg);
                } else {
                    store.push(msg);
                    send(&mut client, &mut store, &mut selflog);
                }
            }
            Err(TryRecvError::Empty) => {
                send(&mut client, &mut store, &mut selflog);
                // blocking explicitly not to spin CPU
                if let Ok(msg) = logs.recv() {
                    store.push(msg);
                }
            }
            Err(TryRecvError::Disconnected) => {
                send(&mut client, &mut store, &mut selflog);
                break;
            }
        };
    }
}

fn send<T: DataDogClient>(
    client: &mut T,
    messages: &mut Vec<DataDogLog>,
    selflog: &mut Option<Sender<String>>,
) {
    match client.send(&messages) {
        Ok(_) => {
            messages.clear();
        }
        Err(e) => {
            if let Some(selflog) = selflog {
                selflog.try_send(e.to_string()).unwrap_or_default();
            }
        }
    }
}
