
# Task 1

The whole application depends on the 3 libraries - `core`, `api` and `web`
The binaries of those each applications are in the `src/bin` where the `Makefile`
shall be used to run all the applications

## The Objective

The objective is to have an application with the following logical architecture:

```
                      +---------+
                      | Web App |
                      +----+----+
                           ^
                           |
          +----------------+------------------+
          |                |                  |
          v                v                  v
   +------------+    +------------+    +------------+
   | API Server |    | API Server |    | API Server |
   +------+-----+    +------+-----+    +------+-----+
          |                 |                 |
          v                 v                 v
  +--------------+  +--------------+  +--------------+
  |  Core Server |  |  Core Server |  |  Core Server |
  +-------+------+  +--------------+  +--------------+
          ^                 ^                 ^
          |                 |                 |
          +-----------------+-----------------+
                   
```

### The Web Application

The web application shall use [rocket framework](https://rocket.rs) for ease of development. It should also contain [VueJS](https://vuejs.org) for the interactivity, as well as [Bootstrap](http://getbootstrap.com) for the UI components

See [Web Application documentation](web/README.md) for the detailed functionalities


### The API Server

The API server is just a `RESTful` app, that can be used as the integration layer to expose the functionality of the `core` server application

### The Core Server

The `core` server is just a key-value store server that has the capabilities of synchronising with other core servers in the space. The information of all `core` servers that are active should be defined at runtime.

