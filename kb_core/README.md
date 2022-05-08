# kb_core

`kb_core` models a logical keyboard and provides implementations for the Keyboard traits.

The purpose of `kb_core` is to implement a pure interface, without depending on the concrete IO used.
As consequence, the keyboard implementations are significantly easier to test.
Furthermore, due to the modular nature of `kb_core`, it's possible to implement virtual keyboards, as well as physical ones.
The `virtual-kb` crate implements a virtual keyboard which binds to the OS event emitting system.

## Concepts

// kb interface is made up of keys
A keyboard is modelled through the `Keyboard` trait, which models a poentially stateful machine which produces outputs based on its inputs.


The Input-Output nature of the keyboard is capture through the `Event` and `Action` type.

An `Event` represnts 3 different scenarios which the keyboard might respond to: KeyPress, KeyRelease and Poll.

An `Action` indicates the 

### Keyboard

