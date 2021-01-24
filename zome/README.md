# Zome Developer Setup

This folder has example DNA for the `interzome` zomes. The actual code for the zomes are in `zomes/zomeA` and `zomes/zomeA`.

To change the code, you can work either opening VSCode inside the root folder of the repo or in this folder, you should have rust intellisense either way.

## Requirements

- Having run through [holochain RSM installation](https://github.com/holochain/holochain-dna-build-tutorial).
- Run all the steps described in this README.md `holochain` core repository.
- Have [`holochain-run-dna`](https://www.npmjs.com/package/@holochain-open-dev/holochain-run-dna) installed globally, and the `lair-keystore` described in its README as well.

## Building

```bash
CARGO_TARGET_DIR=target cargo build --release --target wasm32-unknown-unknown
dna-util -c interzome.dna.workdir/
```

## Testing

After having built the DNA:

```bash
cd test
npm install
npm test
```

## Running

After having built the DNA:

```bash
holochain-run-dna interzome.dna.gz
```

Now `holochain` will be listening at port `8888`;

Restart the command if it fails (flaky holochain start).
