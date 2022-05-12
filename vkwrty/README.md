# Virtual Keywerty
Virtual Keywerty (`vkwrty`) implements a virtual keyboard daemon which binds to the OS key events.
The idea is to intercept key strokes (events) from ones physical keyboard, map it through keywerty's logical key processing and emit key events as result.
The end goal is to give all the benefits of a customizable and progammable keyboard at an OS level.

Currently, it supports only Linux systems however multi-platform support is one goal of the project goals.

## Building
`vkwrty` is a Rust Cargo project which depends on the `kwerty`.
Assuming the repository is cloned, the folder structure has been preserved and that `cargo` is installed locally, building is done through cargo's usual way, that is:
```
cargo build
```

## Running
`vkwrty` needs an event source to bind against for startup, usually that means the event file for the keyboard you'd like to intercept.
In Linux, that means one of the devices under `/dev/input/`.

Currently `vkwrty` does not help you finding out which device you should use.
I recommend using the [`evtest`](https://github.com/freedesktop-unofficial-mirror/evtest) tool and experimenting a bit.

Assuming you've identified your event file and that you are in the root of the "virtual kwerty" project, you can run `vkwrty` with:

```
sudo target/debug/vkwrty /dev/input/event{}
```

where `{}` is the number matching the device you want to intercept.

Note that, currently sudo is required to run `vkwrty`.


## Configuration
This is a big TODO and the number one priority.
Right now, configuring `vkwrty` requires altering the source code.

If you're like to give it a go, checkout the [`main.rs`](vkwrty/main.rs).


## How it works
### Linux
`vkwrty` makes uses of Linux's `uinput` module to intercept to events being emitted by a device, transforms them and generate new keyboard events.
The implementation uses `libevdev` as a wrapper for `uinput` capabilities.

Some useful links to about these Linux resources can be found in the references section.


## Todos
- [ ] Document code
- [ ] Write about envisioned code architecture
- [ ] Define Configuration file syntax (DSL/Json/Yaml)
- [ ] Configuration file support
- [ ] Figure out how to run without sudo or limit capabilities
- [ ] Refactor Runtime to use an OS agnostic interface
- [ ] Add windows and mac OS support?
- [ ] Add live configuration udpate
- [ ] Add interactive device selection

## References
- https://www.freedesktop.org/software/libevdev/doc/latest/index.html
- https://www.kernel.org/doc/html/latest/input/uinput.html
- https://www.kernel.org/doc/html/latest/input/input.html
