# Keywerty
The `Keywerty` (kwrty) project consists of two elements:
- A module that implements a logical keyboard with stateful key activation behaviors.
- Programs that use said module in different domains.

Let's break it down.

## Keywerty module
Reiterating, the module implements a logical keyboard with stateful key activation behaviors.

The "Logical Keyboard" part means that the keyboard is not bound to an underlying IO system, meaning it can be used in different domains, such as firmwares, drivers or user space applications.
Through the core logical implementation, it's possible to apply `Keywerty`'s rich customization potential in all sorts of program.

By "stateful key activation" we mean that upon a key press it may behave differently than usual.
A concrete example: pressing and holding the "Caps Lock" key will be translated to "Ctrl" and tapping the same key just means "Caps Lock".
This sort of conditional / multi action behaviors is what is meant by stateful key activation.
The Mechanical keyboard community is no stranger to this sort of trickery, but they often rely on this customization at firmware level.
Many other "behaviors" exist, the previous case is just an easy to grok example.


## Applications
Due to the flexible nature of the virtual keyboard module, it can be applied to different domains.
One such usage is the "Virtual Keyqwerty" (vkwrty) project.

### Virtual Keywerty
`vkwrty` is an user space application that implements a virtual keyboard.
The idea is that vkwrt is going to intercept key events from ones physical keyboard, map them through the stateful `kwrty` machine and emitt new key events to the underlying OS.
Check out [virtual-keywerty](./virtual-keywerty) for more information
