use super::log::DataDogLog;
use crate::client::AsyncDataDogClient;
use flume::Receiver;
use futures::{task::Poll, Future, FutureExt, Stream, StreamExt};
use std::pin::Pin;

pub struct LoggerFuture<T> {
    receiver: Receiver<DataDogLog>,
    client: T,
    messages: Vec<DataDogLog>,
}

impl<T> LoggerFuture<T>
where
    T: AsyncDataDogClient,
{
    pub fn new(client: T, receiver: Receiver<DataDogLog>) -> Self {
        LoggerFuture {
            receiver,
            client,
            messages: Vec::new(),
        }
    }

    /// Sends message via client.
    ///
    /// Should never return `Poll:Ready(None)`.
    /// Item returned by the future is optional error message to be read by owner of stream.
    fn send_and_poll(&mut self, cx: &mut std::task::Context<'_>) -> Poll<Option<Option<String>>> {
        let poll = Pin::new(
            &mut self.client.send_async(&self.messages).map(|o| match o {
                Ok(_) => None,
                Err(e) => Some(e.to_string()),
            }),
        )
        .poll(cx);

        match poll {
            Poll::Ready(s) => Poll::Pending,
            Poll::Pending => Poll::Pending,
        }
    }
}

impl<T> Stream for LoggerFuture<T>
where
    T: AsyncDataDogClient + Unpin,
{
    /// Values of this stream are diagnostic messages
    type Item = Option<String>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let this = &mut *self;
        let result = loop {
            println!("Inside loop");
            match this.receiver.try_recv() {
                Ok(msg) => {
                    println!("Inside OK");
                    this.messages.push(msg);
                    if this.messages.len() < 50 {
                        // TODO: make it configurable
                        continue;
                    } else {
                        break this.send_and_poll(cx);
                    }
                }
                Err(flume::TryRecvError::Empty) => {
                    println!("Inside EMPTY");
                    if this.messages.is_empty() {
                        // using peekable next element is not consumed
                        // this is field for improvement - how to schedule wakeup in most efficient and correct way?
                        break Pin::new(&mut this.receiver.stream().peekable())
                            .poll_peek(cx)
                            .map(|_| Some(None));
                    } else {
                        break this.send_and_poll(cx);
                    }
                }
                Err(flume::TryRecvError::Disconnected) => {
                    println!("Inside disconnected");
                    if !this.messages.is_empty() {
                        break this.send_and_poll(cx);
                    }
                    // we'll finish the future in the next poll as no new message will arrive
                    break Poll::Ready(None);
                }
            }
        };

        println!("Exited loop");

        result
    }
}
