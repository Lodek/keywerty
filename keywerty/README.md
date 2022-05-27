# Keywerty
The Keywerty module models a logical keyboard and implements a keyboard with stateful key activation behaviors. 
Its goals are to implement a pure interface, without depending on the underlying IO used.
As consequence, the keyboard implementations are significantly easier to test.

Furthermore, since it is not bound to an IO system, it can be used in many different domains such as: firmwares, drivers, user space applications or whatever.

To see one such application, checkout [Virtual Keywerty](../virtual-keywerty), which implements an OS bound virtual keyboard.

## Concepts
The following are some important concepts to understand and use keywerty from a user's perspective.

Effectively, users will be required to define a Keyboard Mapping, therefore it's imporantant to understand the logical model
and `KeyConf` in order to do that.

### Keyboard
Keywerty uses a logical representation of a keyboard.

A Keyboard is a blackbox that takes in `Event`s and it spits outs `Action`s.
An `Event` can be either: a key press, a key release or a poll request.
An `Action` indicates an action to send or to stop sending some data.

`Event`s are bound to `Keys`, where a Key is an unique identifier for one of the keyboard's members.

This simple model is rather versatile and can be used in different domains.

### Key Configurations
As mentioned, keyboards have Keys (an unique Id) and these keys can be configured in different ways.

A `KeyConf` object indicates the activation mode of a key, such as a "Tap Key" or a "Hold Key".
Key configurations make keys "behave" differently.
For instance, a Tap Key will send some action when pressed down and stop sending data when released.
A Hold Key will behave like a Tap Key if quickly tapped but if pressed and held for a threshold, it can send some distinct data.

Each KeyConf variant will store data indicating what the keyboard should do once a key is pressed.

For more information about the available Key Confs checkout [`key.rs`](./src/keys.rs).


### Key Action

A `KeyAction` is like an `Action` but it's internal to the keyboard.
Not everything done by a key should be propaged to the outside world in that sometimes we must mutate the Keyboard itself.
One example of that is when setting a Keyboard Layer.

Each KeyAction specify the mutation that should be performed in the Keyboard, key actions are emitted by keys
once they understand that one of its activation modes have been reached.

### Layer
A layer is a virtual keyboard mapping.

We can think of a normal keyboard as a Map, where the index key is the KeyId and the value is KeyConf, call this Map a Layer.

A keyboard with layers is one where there's a Map of Maps, the map key is the Layer id and the value is a Layer.

This gives us the ability to change how the key behaves but also how it sends dynamically.

This should be somewhat familiar as most laptops have some sort of "fn" key which does that, but to a limited extent.


## Code Structure / diagrams
TODO


## Todos
- [ ] review code docs (docrs)
- [ ] improve tests
- [ ] add examples showing how to use SMKb
- [ ] Bump to 1.0 and publish crate
