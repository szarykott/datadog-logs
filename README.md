# Datadog Logs

This simple crate can be used to log to DataDog directly via HTTP(S).

It is as minimal as possible with the idea that logging should not consume time in your application execution path. Therefore it offloads the task of logging to a separate thread.

As this library is in early stage and to facilitate integration with logging facades such as `log` sending messages to DataDog is done synchronously, but on a dedicated thread in batch fashion.

### `Log` integration

Crate is already integrated with `log` crate, it is hidden behind `log-integration` feature (enabled by default).

### Self logging

To enable self logging, just enable feature `self-log`. Library will then log some trace information to stderr.

### Future features

Contributions welcome:

* try to integrate logger with async executors, so that act of logging is no longer synchronous
* use TCP protocol with TLS instead of HTTPS