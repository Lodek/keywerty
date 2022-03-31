mod virtual_dev;

/*
use evdev_rs::enums::EV_KEY;

pub enum Error {
    CErr(c_int)
}

type Result<T> = std::result::Result<T, Error>;


// not sure about the vector but i'll roll with it
pub enum RuntimeEvent<'a> {
    Report(&'a mut EventReport),
    Poll
}

struct Runtime {
    // create channel
    // create instance of evdev listener, send tx
    // create instance of timer, send tx
    // store thread handles
    // loop
    // block on channel read until timeout or report is available
    // call evdev keyboard and generate event report
    // send report to virtual device
}

impl Runtime {
    fn new() -> Self;
    fn run<Kb>(kb: Kb) 
        where Kb: EvdevKeyboard;
}

struct Timer {
    fn new(period: Duration);
    // loop, sleep, write to tx
    // no need for hardware timer
    // would be cool to use a signal system here
    // but maybe for the future
    fn run(tx: Sender<>);
}
*/
