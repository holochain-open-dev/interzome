# INTERZOME_MODULE

Small demo to show how to make two zomes talk to each other, in holochain RSM.

This module is designed to be help people do basic things. It is packaged as two holochain zomes, no UI is given, it is tryorama only

## Documentation

this is it!

## Assumptions

none

## Installation and usage


```
the demo uses the following revisions:

hc_utils = {git = "https://github.com/holochain/hc-utils", branch = "develop", package = "hc_utils"}
hdk3 = {git = "https://github.com/holochain/holochain", rev = "3675b58", package = "hdk3"}
holo_hash = {git = "https://github.com/holochain/holochain", rev = "3675b58", package = "holo_hash"}
```


Take into account that at this point the elements already expect a holochain conductor running at `ws://localhost:8888`.

## Developer setup

This respository is structured in the following way:

- `zome/`: example DNA with the `interzome` code.
- Top level `Cargo.toml` is a virtual package necessary for other DNAs to include this zome by pointing to this git repository.

Read the [Zome developer setup](/zome/README.md).
