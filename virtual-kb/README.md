# virtual-kb
Linux based virtual keyboard daemon.

The virtual keyboard listens to events being emitted by a device, transforms them and generate new keyboard events.
The implementation is based on `libevdev`.
`libevdev` can listen to device files and events can be generated through it by making use of `uinput` module.

The goal of this project is to combine the daemon with a stateful keyboard implementation, such as `statem_keyboard`, and implement stateful key activation modes, such as tap and hold.

Some useful links to understand the implementation can be found in the references section


## Todos
[ ] Document code
[ ] Write about envisioned code architecture
[ ] Define Configuration file syntax (DSL/Json/Yaml)
[ ] Configuration file support
[ ] Refactor Runtime to use an OS agnostic interface
[ ] Add windows and mac OS support?
[ ] Add live configuration udpate

## References
- https://www.freedesktop.org/software/libevdev/doc/latest/index.html
- https://www.kernel.org/doc/html/latest/input/uinput.html
- https://www.kernel.org/doc/html/latest/input/input.html
