#[derive(PartialEq)]
pub enum BlockReason {
    Delay(usize),         // 延时阻塞，参数为剩余tick数
    Signal(SignalType), // 信号阻塞，等待特定信号
    Mutex(usize),     // 互斥量阻塞
}

#[derive(PartialEq, Copy, Clone)]
pub enum SignalType {
    Timer,            // 定时器信号
    External,         // 外部信号
    UserDefined(usize), // 用户自定义信号
}

impl From<usize> for SignalType {
    fn from(value: usize) -> Self {
        match value {
            0 => SignalType::Timer,
            1 => SignalType::External,
            _ => SignalType::UserDefined(value),
        }
    }
}

impl Into<usize> for SignalType {
    fn into(self) -> usize {
        match self {
            SignalType::Timer => 0,
            SignalType::External => 1,
            SignalType::UserDefined(value) => value,
        }
    }
}
