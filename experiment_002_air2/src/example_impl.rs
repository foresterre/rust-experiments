use crate::{Id, Reporter};
use std::io::Write;
use std::sync::{Arc, Mutex};

pub enum MyProgressEvents {
    Reset,
    Inc { value: u64 },
    Finish,
}

pub enum MyEvents {
    WriteLine(String),
}

// ------------

pub struct HumanReporter {
    progress: indicatif::ProgressBar,
}

impl HumanReporter {
    pub fn new() -> Self {
        let mut bar = indicatif::ProgressBar::new(100);
        bar.enable_steady_tick(200);

        Self { progress: bar }
    }
}

impl Reporter for HumanReporter {
    type ProgressEvent = MyProgressEvents;
    type Event = MyEvents;

    fn send_progress(&self, event: Self::ProgressEvent) -> crate::Result<()> {
        match event {
            MyProgressEvents::Reset => self.progress.reset(),
            MyProgressEvents::Inc { value } => self.progress.inc(value),
            MyProgressEvents::Finish => self.progress.finish(),
        }

        Ok(())
    }

    fn send_event(&self, event: Self::Event) -> crate::Result<()> {
        match event {
            MyEvents::WriteLine(msg) => self.progress.println(msg),
        }

        Ok(())
    }
}

pub struct JsonReporter<W: Write> {
    writer: Arc<Mutex<W>>,
    progress: Arc<Mutex<Progress>>,
}

impl<W: Write> JsonReporter<W> {
    pub fn new(writer: W) -> Self {
        Self {
            writer: Arc::new(Mutex::new(writer)),
            progress: Arc::new(Mutex::new(Progress::new(100))),
        }
    }
}

impl<W: Write> Reporter for JsonReporter<W> {
    type ProgressEvent = MyProgressEvents;
    type Event = MyEvents;

    fn send_progress(&self, event: Self::ProgressEvent) -> crate::Result<()> {
        let mut writer = self.writer.lock().unwrap();

        match event {
            MyProgressEvents::Reset => {
                let mut guard = self.progress.lock().unwrap();
                guard.current = 0;

                writer.write_all(&message(&guard).as_bytes());
            }
            MyProgressEvents::Inc { value } => {
                let mut guard = self.progress.lock().unwrap();
                guard.current += value;

                writer.write_all(&message(&guard).as_bytes());
            }
            MyProgressEvents::Finish => {
                let mut guard = self.progress.lock().unwrap();
                guard.current = guard.max;

                writer.write_all(&message(&guard).as_bytes());
            }
        }

        writer.flush();

        Ok(())
    }

    fn send_event(&self, event: Self::Event) -> crate::Result<()> {
        Ok(())
    }
}

// derive Json
struct Progress {
    current: u64,
    max: u64,
}

impl Progress {
    fn new(max: u64) -> Self {
        Self { current: 0, max }
    }

    fn inc(&mut self, by: u64) {
        self.current += by;
    }
}

fn message(value: &Progress) -> String {
    format!(
        "{{ 'progress' : {{ 'current': {}, 'max': {} }} }}\n",
        value.current, value.max
    )
}
