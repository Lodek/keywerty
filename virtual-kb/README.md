# virtual-kb-linux
Linux based virtual keyboard daemon.

The virtual keyboard listens to events being emitted by a device, transforms them and generate new events using Linux's `uinput` module.

The goal of this project is to combine the daemon with a stateful keyboard implementation, such as `statem_keyboard`, and implement stateful key activation modes, such as tap and hold.
