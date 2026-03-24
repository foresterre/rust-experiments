use crate::example_impl::{MyEvents, MyProgressEvents};
use std::fmt::{Display, Formatter};
use std::os::unix::raw::mode_t;

#[derive(Debug)]
struct Error;

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("yikes, err!")
    }
}

mod example_impl;

type Result<T> = std::result::Result<T, Error>;
#[derive(Debug, Copy, Clone)]
struct Id(u32);

trait Reporter {
    type ProgressEvent;
    type Event;

    fn send_progress(&self, event: Self::ProgressEvent) -> Result<()>;

    fn send_event(&self, event: Self::Event) -> Result<()>;
}

// ==================

fn main() -> Result<()> {
    use example_impl::HumanReporter;
    use example_impl::JsonReporter;

    let reporter = HumanReporter::new();

    std::thread::spawn(move || {
        let _ = do_stuff(&reporter);
    });

    let stdout = std::io::stdout();
    let writer = stdout.lock();
    let reporter = JsonReporter::new(writer);

    let _ = do_stuff(&reporter);

    Ok(())
}

fn do_stuff<R: Reporter<Event = MyEvents, ProgressEvent = MyProgressEvents>>(
    reporter: &R,
) -> Result<()> {
    std::thread::sleep_ms(1000);

    reporter.send_event(MyEvents::WriteLine("Hello chris!".into()))?;
    reporter.send_progress(MyProgressEvents::Inc { value: 10 });
    reporter.send_progress(MyProgressEvents::Inc { value: 10 });

    std::thread::sleep_ms(1000);

    reporter.send_progress(MyProgressEvents::Inc { value: 10 });
    reporter.send_event(MyEvents::WriteLine("Hello jean!".into()))?;
    reporter.send_progress(MyProgressEvents::Inc { value: 10 });

    std::thread::sleep_ms(1000);

    reporter.send_event(MyEvents::WriteLine("Hello more chris!".into()))?;
    reporter.send_progress(MyProgressEvents::Inc { value: 10 });
    reporter.send_progress(MyProgressEvents::Finish);

    Ok(())
}
