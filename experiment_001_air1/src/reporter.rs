use crate::{Event, Report, ReportType};
use std::marker::PhantomData;

/// A reporter, which writes messages of type `T` to a writer `W`.
#[derive(Debug)]
pub struct Reporter<T: ReportType, W> {
    output_type: PhantomData<T>,
    writer: W,
}

impl<T: ReportType, W> Reporter<T, W> {
    pub fn new(writer: W) -> Self {
        Self {
            output_type: PhantomData,
            writer,
        }
    }
}

impl<T, W> Report<T> for Reporter<T, W>
where
    T: ReportType,
    W: std::io::Write,
{
    fn report_event<E>(&mut self, event: E)
    where
        E: Event<T>,
    {
        let writer = &mut self.writer;
        event.write_fmt(writer);
    }
}
