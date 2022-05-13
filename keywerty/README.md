# Keywerty
The Keywerty module models a logical keyboard and implements a keyboard with stateful key activation behaviors. 
Its goals are to implement a pure interface, without depending on the underlying IO used.
As consequence, the keyboard implementations are significantly easier to test.

Furthermore, since it is not bound to an IO system, it can be used in many different domains such as: firmwares, drivers, user space applications or whatever.

To see one such application, checkout [Virtual Keywerty](../virtual-keywerty), which implements an OS bound virtual keyboard.

## Activation behaviors
TODO

## Concepts
TODO

# Todos
- [ ] write about activation behaviors and data model
- [ ] Try docrs
- [ ] Write stronger requirements for state machine trait
- [ ] write tests
- [ ] refactor / finish other state machines
