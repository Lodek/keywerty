struct EventReport {}

/// Listen to an event device and generates reports
/// Responsible for correctly handling syn drop semantics.
/// Listener must be non block
trait EvdevListener {
    fn new(device: Path) -> Self;
    fn generate_report() -> Option<EventReport>
}


/// Represents a virtual uinput device.
/// Initializes device and emits events
trait VirtualDevice {
    fn new() -> Result<Self>;
    fn publish_events(report: EventReport) -> Result<()>;
}

/// Wraps around multi action keyboard trait 
/// Receives a complete event report without sync events
/// return full report containing sync events
trait EvdevKeyboard: MultiActionKeyboard {
    fn evdev_transition(&EventReport) -> EventReport;
}

// The runtime has to be an event loop
// there will be two types of events, one for fd updates
// and one for timers
// compare select and epoll for event loop
// implement multiple runtimes, based on async with tokio and manual event loop

// Can I make these data types trait-based, such that it might be extended for other operating
// systems?
