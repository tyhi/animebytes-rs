[![Build Status](https://tc.tyhi.rs/app/rest/builds/buildType:(id:animebytes_build)/statusIcon)]()
# animebytes-rs
This crate is for accessing the animebytes.tv tracker's rest API. It covers all the currently available endpoints.


## Quickstart
```rust
use animebytes_rs::Client;

let encoder = Client::new("AB_KEY", "AB_USER").unwrap();

let search_result = client.search_anime("sword art online").await.unwrap();
```

# License
Licensed under MIT