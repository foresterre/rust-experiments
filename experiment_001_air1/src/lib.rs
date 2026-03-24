mod reporter;

pub use reporter::Reporter;

/// Trait which handle reporting an event, to be implemented by a specific reporter
pub trait Report<T> {
    fn report_event<E>(&mut self, event: E)
    where
        T: ReportType,
        E: Event<T>;
}

pub trait ReportType {}

pub trait Event<T: ReportType> {
    fn write_fmt<W>(&self, writer: &mut W)
    where
        W: std::io::Write;
}
