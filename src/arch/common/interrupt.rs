pub trait InterruptController {
    fn enable_interrupt(irq: InterruptNumber);
    fn disable_interrupt(irq: InterruptNumber);
    fn set_priority(irq: InterruptNumber, priority: u8);
}