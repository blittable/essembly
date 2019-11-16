## Overview
essembly-test is a set of integration tests run from the command line and can be used with CI/CD 

It overlaps a bit with essembly-cli, the reference client.  

Generally, the cli tests should be specific to application functionality (connection/input/output).  The tests here are more general integration tests.

To run:

From the essembly-test directory (not the root)
`cargo test`
