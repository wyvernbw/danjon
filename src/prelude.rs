use std::{fmt::Display, process::exit, str::FromStr, sync::Arc};

use inquire::{InquireError, Select};
use yansi::Paint;

/// newtype pattern
#[derive(Clone)]
pub struct W<T>(pub T);

fn or_cancel<T>(res: Result<T, InquireError>) -> anyhow::Result<T> {
    res.map_err(|err| match err {
        InquireError::OperationCanceled | InquireError::OperationInterrupted => {
            exit(0);
        }
        err => anyhow::Error::new(err),
    })
}

fn or_retry<T>(init: anyhow::Result<T>, f: impl Fn() -> T) -> T {
    match init {
        Ok(val) => val,
        Err(err) => {
            tracing::error!(?err);
            println!("{}", Paint::red(err));
            f()
        }
    }
}

pub fn try_select<T: Display>(prompt: &str, opts: Vec<T>) -> anyhow::Result<T> {
    or_cancel(Select::new(prompt, opts).prompt())
}

pub fn select<T: Display + Clone>(prompt: &str, opts: Vec<T>) -> T {
    or_retry(try_select(prompt, opts.clone()), || {
        select(prompt, opts.clone())
    })
}

pub fn try_input(prompt: &str) -> anyhow::Result<String> {
    or_cancel(inquire::Text::new(prompt).prompt())
}

pub fn input(prompt: &str) -> String {
    or_retry(try_input(prompt), || input(prompt))
}

pub fn input_map<T, Res: IntoAnyhow<T>>(prompt: &str, f: impl Fn(&str) -> Res + Clone) -> T {
    let try_input = || try_input(prompt).and_then(|s| f(&s).anyhow());
    or_retry(try_input(), || input_map(prompt, f.clone()))
}

pub fn try_confirm(prompt: &str) -> anyhow::Result<bool> {
    or_cancel(
        inquire::Confirm::new(prompt)
            .with_error_message(
                &Paint::red("Invalid answer, try typing 'y' for yes and 'n' for no").to_string(),
            )
            .prompt(),
    )
}

pub fn confirm(prompt: &str) -> bool {
    or_retry(try_confirm(prompt), || confirm(prompt))
}

pub fn arc_str(string: String) -> Arc<str> {
    Arc::from(string)
}

impl FromStr for W<Arc<str>> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(W(Arc::from(s)))
    }
}

impl ToString for W<Arc<str>> {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

pub trait IntoAnyhow<T> {
    fn anyhow(self) -> anyhow::Result<T>;
}

impl<T, E: std::error::Error + Send + Sync + 'static> IntoAnyhow<T> for Result<T, E> {
    fn anyhow(self) -> anyhow::Result<T> {
        self.map_err(|e| anyhow::Error::from(e))
    }
}

pub use crate::testing::test;
pub use crate::testing::TResult;
