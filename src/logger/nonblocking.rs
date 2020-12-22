use super::log::DataDogLog;
use crate::client::AsyncDataDogClient;
use flume::{Receiver, RecvError, Sender, TryRecvError};
use futures::Future;

// TODO: Reduce code duplication
pub fn logger_future<T>(
    mut client: T,
    logs: Receiver<DataDogLog>,
    mut selflog: Option<Sender<String>>,
) -> impl Future<Output = ()>
where
    T: AsyncDataDogClient,
{
    async move {
        let mut store = Vec::new();
        loop {
            match logs.try_recv() {
                Ok(msg) => {
                    println!("got message");
                    if store.len() < 50 {
                        store.push(msg);
                        continue;
                    } else {
                        store.push(msg);
                        send(&mut client, &mut store, &mut selflog).await;
                    }
                }
                Err(TryRecvError::Empty) => {
                    println!("in empty");
                    if !store.is_empty() {
                        send(&mut client, &mut store, &mut selflog).await;
                    }
                    println!("awaitning next item in empty");
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
                    println!("in disconnected");
                    if !store.is_empty() {
                        send(&mut client, &mut store, &mut selflog).await;
                    }
                    break ();
                }
            };
        }
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
