use std::time::Instant;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToastKind {
    Success,
    Error,
    Info,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToastDuration {
    Timed(u64),
    Persistant,
}

#[derive(Debug, Clone)]
pub struct Toast {
    pub id: u64,
    pub message: String,
    pub kind: ToastKind,
    pub born: Instant,
    pub duration: ToastDuration,
}

impl Toast {
    pub fn success(msg: impl Into<String>) -> Self {
        Self::new(msg, ToastKind::Success, ToastDuration::Timed(3000))
    }

    pub fn _error(msg: impl Into<String>) -> Self {
        Self::new(msg, ToastKind::Error, ToastDuration::Timed(5000))
    }

    pub fn _info(msg: impl Into<String>) -> Self {
        Self::new(msg, ToastKind::Info, ToastDuration::Timed(3000))
    }

    pub fn persistent(msg: impl Into<String>, kind: ToastKind) -> Self {
        Self::new(msg, kind, ToastDuration::Persistant)
    }

    pub fn is_expired(&self) -> bool {
        match self.duration {
            ToastDuration::Persistant => false,
            ToastDuration::Timed(ms) => {
                self.born.elapsed().as_millis() as u64 >= ms
            }
        }
    }

    fn new(
        msg: impl Into<String>,
        kind: ToastKind,
        duration: ToastDuration,
    ) -> Self {
        Self {
            id: 0,
            message: msg.into(),
            kind,
            born: Instant::now(),
            duration,
        }
    }
}
