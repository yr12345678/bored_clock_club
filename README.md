This repo demonstrates an issue with `Clock::current_time_rounded_to_minutes()` returning 0 during tests using TestEnvironment (and perhaps LedgerSimulator, but this has not been tested), even though the current ledger time has advanced beyond 0.

Run `scrypto test` to run a simple test which asserts Instants returned from using, respectively, `Clock::current_time_rounded_to_minutes()` and `Clock::current_time_rounded_to_seconds()`, are equal.

The test fails with the following error:

```
---- returned_times_are_equal stdout ----
thread 'returned_times_are_equal' panicked at tests/lib.rs:20:5:
assertion `left == right` failed: Times were not equal
  left: Instant { seconds_since_unix_epoch: 0 }
 right: Instant { seconds_since_unix_epoch: 2419200 }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```
Where `left` is the Instant returned from `Clock::current_time_rounded_to_minutes()` and `right` is the Instant returned from `Clock::current_time_rounded_to_seconds()` 

This was tested on MacOS using Rust 1.77.2 and Scrypto 1.2.0
