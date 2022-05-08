# shorty

Small link shortener

An instance is currently hosted at https://s.u-are.gay
if you want to try it out :)


## Self hosting  

Currently shorty is intended to be run behind a reverse proxy to take advantage of multiple URLs and
https. I personally use nginx, but you can use any reverse proxy you want.

Theres multiple ways to run shorty, you can either grab a pre-compiled binary (from a release or recent 
pipeline run) or you can compile it yourself.

### Self compiling
If you have decided to compile it yourself you need to clone this repository.
You will also need a functioning [rust toolchain](https://www.rust-lang.org/tools/install).

Once both of these requirements are met, you can compile shorty with `cargo build --release`.
After the compilation is done you can find the resulting binary in the `target/release` folder
which should have appeared.


### Configuration
A sample config with the name `config.toml.sample` should be provided.
Copy it and rename it to `config.toml` and then replace the placeholder values with the details
for your configuration.
