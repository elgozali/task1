
# Web Documentation

The `web` application shall only contain a call using `Javascript` to the `REST` endpoints specific to the `core` node.

The `wireframe` UI should look something like below, in this case, `Node 1` is selected as the main node where the write operation shall commence.

The `real-time value` window shall swap against the chosen `core node`, i.e.
if the chosen node is `Node 2`, the value listed should be `Node 1`, `Node 3` and `Node 4` and vice versa.

The `Core Node` chosen should be using the `HTML select` from the [Bootstrap](http://getbootstrap.com) library.


```
+--------------------------------------------------------+
|            +--------------+             +----------+   |
| Set Value: | (text field) |  Core Node: | Node 1 |V|   |
|            +--------------+             | Node 2   |   |
|                                         | Node 3   |   |
|                                         | Node 4   |   |
|                                         +----------+   |
+--------------------------------------------------------+
| Real-time Value                                        |
|    Node 2: {{ value }}                                 |
|    Node 3: {{ value }}                                 |
|    Node 4: {{ value }}                                 |
+--------------------------------------------------------+


```