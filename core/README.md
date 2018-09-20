
## Core Documentation

The core application is the one that handles the synchronization of values in the `in memory` database. Recall that any update to one server should initiate a flurry of updates to other `core` nodes

```
         +---------------+-----------------\
         |               |                 |
         |               V                 V
  +------+------+  +-------------+  +-------------+
  | Core Server |  | Core Server |  | Core Server |
  +-------------+  +------+------+  +-------+-----+
         ^                ^                 |
         |                |                 |
         \----------------+-----------------+
```

## The Objective

The value that is written to one node shall be broadcasted to other nodes so that their values are equal. We can ignore the `race conditions` for now as the test shall be done **ONLY** via the `web` application.

The core app should be coded using predominantly `tokio` library. Click [tokio toolkit](https://github.com/tokio-rs/tokio), or [tokio website](https://tokio.rs) for more info.
