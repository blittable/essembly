## The `essembly-core` crate contains all of the business non-application business logic for essembly. The root of the library controls permissions.  

Those permissions are required by all the sub-crates (inventory, accounting, etc.)  For now, all sub-crates are defined as optional.  Logging is a core library. 


`essembly-core` knows nothing about servers, apis, protocols, clients, etc.  It represents business primitives.

