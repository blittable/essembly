## The `essembly-logger` crate contains all of the logging logic for essembly. 


There are two primary users:

1. The system administrator configuring an implementation.
2. The end-user of a standalone application.

From the end-users' perspective, logging should be simple.  There is a configuration file like this:

```
[traffic-cop]
primary = { ip = "localhost", port = "2888" }
secondary = { ip = "222.222.222.2", port = "2888" }

[cli]
primary = { ip = "localhost", port = "2234", logging = "trace" }
secondary = { ip = "localhost", port = "2235", logging = "trace" }

[api]
primary = { ip = "localhost", port = "2234", logging = "trace" }
secondary = { ip = "localhost", port = "2235", logging = "trace" }

[db]
primary = {db-type = "sled",  ip = "localhost", port = "2234", logging = "trace" }
secondary = {db-type = "sled",  ip = "localhost", port = "2334", logging = "trace" }

[logger]
local = { directory = "/var/lib/vector" }
remote = { ip = "123.333.333.23", port = "2234" }
```

Each component in the system can be configured for logging with different levels of verbosity.  

After a component is started, the logging should happen automatically.   

