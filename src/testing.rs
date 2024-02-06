use std::io::Write;

use strip_ansi_escapes::strip_str;
use yansi::Paint;

pub trait TestResult: core::fmt::Debug {}

struct InnerTestWriter;

impl InnerTestWriter {
    pub fn new() -> Self {
        Self
    }
}

impl Write for InnerTestWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let msg = std::str::from_utf8(buf).unwrap();
        let msg = strip_str(msg);
        let s = "\t".to_string() + &Paint::new(msg).dimmed().italic().to_string();
        std::io::stdout().write_all(s.as_bytes()).unwrap();
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn() -> TResult]) {
    use tiny_gradient::{Gradient, GradientStr, RGB};

    use tracing::span;
    use tracing_subscriber::{layer::SubscriberExt, Layer};

    let inner_test_layer = tracing_subscriber::fmt::layer()
        .compact()
        .without_time()
        .with_file(true)
        .with_line_number(true)
        .with_test_writer()
        .with_writer(InnerTestWriter::new)
        .with_filter(tracing_subscriber::filter::FilterFn::new(|metadata| {
            metadata.target() != "danjon::testing"
        }));
    let stdout = tracing_subscriber::fmt::layer()
        .pretty()
        .without_time()
        .with_file(false)
        .with_level(false)
        .with_file(false)
        .with_line_number(false)
        .with_test_writer()
        .with_filter(tracing_subscriber::filter::FilterFn::new(|metadata| {
            metadata.target() == "danjon::testing"
        }));
    let global_subscriber = tracing_subscriber::Registry::default()
        .with(stdout)
        .with(inner_test_layer);

    tracing::subscriber::set_global_default(global_subscriber).unwrap();

    for (i, test) in tests.iter().enumerate() {
        tracing::info!("Running test #{}...", i);
        std::panic::set_hook(Box::new(move |info| {
            let err: String = format!("{}", info).replace('\n', " ");
            let msg = Paint::new(format!("\nTest #{} failed: \n\t{}", i, err))
                .bold()
                .fg(yansi::Color::Red);
            tracing::error!("{}", msg);
        }));
        let now = std::time::Instant::now();
        let result = span!(tracing::Level::TRACE, "Inner test").in_scope(test);
        println!();
        let elapsed = now.elapsed();
        let ok = Paint::new("ok").fg(yansi::Color::Green).bold();
        tracing::info!(?result, ?elapsed, "Test #{i} ... {ok}")
    }
    tracing::info!(
        "{}",
        "✨ All tests passed!".gradient([
            RGB::new(197, 249, 215),
            RGB::new(247, 212, 134),
            RGB::new(242, 122, 125)
        ])
    );
}

impl<T> TestResult for T where T: core::fmt::Debug {}

impl<T> From<anyhow::Result<T>> for Box<dyn TestResult> {
    fn from(res: anyhow::Result<T>) -> Self {
        Box::new(res.map(|_| ()) as Result<(), _>)
    }
}

impl From<()> for Box<dyn TestResult> {
    fn from(_: ()) -> Self {
        Box::new(())
    }
}

pub type TResult = Box<dyn TestResult>;

pub fn test<T>(f: impl Fn() -> T) -> TResult
where
    T: TestResult + 'static,
{
    Box::new(f()) as TResult
}
