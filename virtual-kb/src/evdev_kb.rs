/// Wraps around multi action keyboard trait 
/// Receives a complete event report without sync events
/// return full report containing sync events
trait EvdevKeyboard: MultiActionKeyboard {
    fn evdev_transition(&EventReport) -> EventReport;
}
