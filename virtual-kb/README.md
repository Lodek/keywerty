# virtual-kb
Linux based virtual keyboard daemon.

The virtual keyboard listens to events being emitted by a device, transforms them and generate new keyboard events.
The implementation is based on `libevdev`.
`libevdev` can listen to device files and events can be generated through it by making use of `uinput` module.

The goal of this project is to combine the daemon with a stateful keyboard implementation, such as `statem_keyboard`, and implement stateful key activation modes, such as tap and hold.

Some useful links to understand the implementation can be found in the references section


## Caveats
Listening to an evdev event emitter does not consume the events.
This can be problematic if a virtual keyboard is used, as that would duplicated events.

Currently, to work around this limitation I recommend configuring your X server to ignore events emitted from your original keyboard.
This can be configured through at the `xorg.conf.d` configuration file using the `Ignore` [directive](https://man.archlinux.org/man/xorg.conf.d.5#Option~40).


## References
- https://www.freedesktop.org/software/libevdev/doc/latest/index.html
- https://www.kernel.org/doc/html/latest/input/uinput.html
- https://www.kernel.org/doc/html/latest/input/input.html
