## Testing `rust-bert`

This is a minimal test on using [rust-bert](https://crates.io/crates/rust-bert) and a model from huggingface.co.

<br/>

### Prereqs

As prerequisites, you need to have `libtorch` installed.<br/>
This version (`0.21.0`) of `rust-bert` works with `libtorch` version `v2.0.0`. Download it from [here](https://download.pytorch.org/libtorch/cu118/libtorch-cxx11-abi-shared-with-deps-2.0.0%2Bcu118.zip) and have it extracted on a directory. Then set or update your environment variables as following (the Linux perspective):

```shell
export LIBTORCH=/path/to/libtorch
export LD_LIBRARY_PATH=${LIBTORCH}/lib:$LD_LIBRARY_PATH
```

<br/>

### Usage

Use the classic `cargo run`.

At first run, it take some time to download the model and its other parts:

```
Downloading https://huggingface.co/EleutherAI/gpt-neo-2.7B/resolve/main/vocab.json [779.45KiB]... ✓ Done! Finished in 0 seconds
Downloading https://huggingface.co/EleutherAI/gpt-neo-2.7B/resolve/main/rust_model.ot [9.94GiB].... ✓ Done! Finished in 4 minutes
```

<br/>
