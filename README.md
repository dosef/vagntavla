# Purpose
Simple departure board for Västtrafik. Mainly done to practice the Rust
language.

# Usage
* The following two environment variables needs to be set which can be retreived
by setting up an account at [Västtrafik Development
Portal](https://developer.vasttrafik.se/portal/#/):

    ```
    VASTTRAFIK_KEY
    VASTTRAFIK_SECRET
    ```

* The default port used is 8000, but can be changed by setting the PORT
environment variable.

* Executing `cargo run` will start the server.

# Example Server
https://vagntavla.herokuapp.com/
