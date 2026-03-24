use air1::{Event, Report, ReportType, Reporter};
use indicatif::ProgressStyle;
use std::io::{Stdout, Write};
use std::marker::PhantomData;

struct MyEvent {
    name: String,
}

pub struct HumanOutput;
pub struct JsonOutput;
pub struct OutputDisabled;

impl ReportType for HumanOutput {}
impl ReportType for JsonOutput {}
impl ReportType for OutputDisabled {}

impl Event<HumanOutput> for MyEvent {
    fn write_fmt<W>(&self, _writer: &mut W)
    where
        W: Write,
    {
        let _ = _writer.write_fmt(format_args!("Hi {} 👋", &self.name));
    }
}
impl Event<JsonOutput> for MyEvent {
    fn write_fmt<W>(&self, _writer: &mut W)
    where
        W: Write,
    {
        let object = json::object! {
            "name": self.name.as_str()
        };

        let _ = _writer.write_fmt(format_args!("{}", object));
    }
}

struct IndicatifReporter<T: ReportType> {
    bar: indicatif::ProgressBar,
    i: u64,
    phantom: PhantomData<T>,
}

impl<T: ReportType> Default for IndicatifReporter<T> {
    fn default() -> Self {
        let bar = indicatif::ProgressBar::new(10);
        bar.enable_steady_tick(100);

        Self {
            bar,
            i: 0,
            phantom: PhantomData,
        }
    }
}

impl<T: ReportType> Report<T> for IndicatifReporter<T> {
    fn report_event<E>(&mut self, event: E)
    where
        T: ReportType,
        E: Event<T>,
    {
        let mut message = Vec::<u8>::new();
        event.write_fmt(&mut message);

        self.bar.println(format!(
            "indicatif [{}]: {}",
            self.i,
            String::from_utf8_lossy(&message)
        ));

        let color = match self.i % 7 {
            0 => "red",
            1 => "orange",
            2 => "yellow",
            3 => "green",
            4 => "blue",
            5 => "purple",
            _ => "pink",
        };

        let style = format!("{{bar:40.{}/{}}} {{pos:>4}}/{{len:4}}", color, color);

        self.bar
            .set_style(ProgressStyle::default_bar().template(&style));

        self.bar.set_position(self.i);
        self.i += 1;
    }
}

fn main() {
    let mut choice = String::with_capacity(32);
    {
        // let _ = std::io::stdin().lock().read_line(&mut choice);
        choice.push_str("bar");
    }

    match choice.as_str().trim() {
        "json" => {
            let mut r = Reporter::<JsonOutput, Stdout>::new(std::io::stdout());
            run_program(&mut r);
        }
        "bar" => {
            let mut r = IndicatifReporter::<HumanOutput>::default();
            run_program(&mut r);
        }
        _ => {
            let mut r = Reporter::<HumanOutput, Stdout>::new(std::io::stdout());
            run_program(&mut r);
        }
    }
}

fn run_program<T: ReportType, R: Report<T>>(reporter: &mut R)
where
    MyEvent: Event<T>, // TODO: figure out how this extra constraint is unnecessary.
{
    for _ in 0..10 {
        let event = MyEvent {
            name: "Christopher".into(),
        };

        reporter.report_event(event);

        std::thread::sleep(std::time::Duration::new(0, 500_000_000));
    }
}
