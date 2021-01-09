use super::log::DataDogLog;
use crate::client::AsyncDataDogClient;
use flume::{Receiver, RecvError, Sender, TryRecvError};

pub(crate) async fn logger_future<T>(
    mut client: T,
    logs: Receiver<DataDogLog>,
    mut selflog: Option<Sender<String>>,
) where
    T: AsyncDataDogClient,
{
    let mut store = Vec::new();
    loop {
        match logs.try_recv() {
            Ok(msg) => {
                if store.len() < 50 {
                    store.push(msg);
                    continue;
                } else {
                    store.push(msg);
                    send(&mut client, &mut store, &mut selflog).await;
                }
            }
            Err(TryRecvError::Empty) => {
                if !store.is_empty() {
                    send(&mut client, &mut store, &mut selflog).await;
                }
                // a trick not to spin endlessly on empty receiver
                match logs.recv_async().await {
                    Ok(msg) => {
                        if store.len() < 50 {
                            store.push(msg);
                            continue;
                        } else {
                            store.push(msg);
                            send(&mut client, &mut store, &mut selflog).await;
                        }
                    }
                    Err(RecvError::Disconnected) => {
                        if !store.is_empty() {
                            send(&mut client, &mut store, &mut selflog).await;
                        }
                        break ();
                    }
                }
            }
            Err(TryRecvError::Disconnected) => {
                if !store.is_empty() {
                    send(&mut client, &mut store, &mut selflog).await;
                }
                break ();
            }
        };
    }
}

async fn send<T>(client: &mut T, logs: &mut Vec<DataDogLog>, selflog: &mut Option<Sender<String>>)
where
    T: AsyncDataDogClient,
{
    if let Err(e) = client.send_async(&logs).await {
        if let Some(selflog) = selflog {
            selflog.send_async(e.to_string()).await.unwrap_or_default()
        }
    } else {
        logs.clear();
    }
}
