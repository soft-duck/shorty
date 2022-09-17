# shorty

Small link shortener

An instance is currently hosted at https://s.u-are.gay
if you want to try it out :)

- [Self hosting](#self-hosting)
  - [Self compiling](#self-compiling)
  - [Configuration](#configuration)
  - [Reverse proxy](#reverse-proxy)
- [Other things](#other-things)


## Self-hosting

Currently shorty is intended to be run behind a reverse proxy to take advantage of multiple URLs and
https. I personally use nginx, but you can use any reverse proxy you want.

There's multiple ways to run shorty, you can either grab a pre-compiled binary (from a release for example)
or you can compile it yourself.

### Self compiling

If you have decided to compile it yourself you need to clone this repository.
You will also need a functioning [rust toolchain](https://www.rust-lang.org/tools/install).

Once both of these requirements are met, you can compile shorty with `cargo build --release`.
After the compilation is done you can find the resulting binary in the `target/release` folder
which should have appeared.

### Configuration

A sample config with the name `config.toml.sample` is provided in this repository. Alternatively, if it
can't find a config file,shorty will create the sample at the config location.
The config location by default is next to the binary, but can be customized via the `SHORTY_CONFIG`
environment variable.

#### Environment Variables
A few things can also be configured via environment variables.
Generally most things that can be configured via an environment variable can also be configured via the config file.
Shorty will also look for a .env file in the same directory it's executed from and pull environment variables from there.

`SHORTY_CONFIG` was mentioned before. With it you can set a custom location for the config file. 
By default shorty will look for the config file in the current folder. This can obviously not be configured 
via the config file :P

`SHORTY_WEBSITE`. With this environment variable you can set the location of a custom frontend which
should be served instead of the default embedded one.


As time goes on more things might be configurable via environment variable.

### Reverse Proxy
If you want HTTPS you currently have to run shorty behind a reverse proxy like nginx.
This might change in the future when/if shorty gets the ability to handle HTTPS by itself. If you don't 
want or need HTTPS then you can also just run shorty on its own.
Another reason for a reverse proxy would be hosting shorty alongside other things that require the HTTP(S) 
port, like a website or another HTTP service.

I personally use nginx but any other reverse proxy should work as well.
There is a sample nginx config included in the repository [here](meta/shorty.conf).

# Other things
If there are any questions or other things you would like to talk about, 
there is a matrix room at `#shorty:matrix.netflam.de`

