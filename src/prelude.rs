use std::{fmt::Display, process::exit, str::FromStr, sync::Arc};

use inquire::{InquireError, Select};

/// newtype pattern
#[derive(Clone)]
pub struct W<T>(pub T);

pub fn or_cancel<T>(res: Result<T, InquireError>) -> anyhow::Result<T> {
    res.map_err(|err| match err {
        InquireError::OperationCanceled | InquireError::OperationInterrupted => {
            exit(0);
        }
        err => anyhow::Error::new(err),
    })
}

pub fn select<T: Display>(prompt: &str, opts: Vec<T>) -> anyhow::Result<T> {
    or_cancel(Select::new(prompt, opts).prompt())
}

pub fn input(prompt: &str) -> anyhow::Result<String> {
    or_cancel(inquire::Text::new(prompt).prompt())
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
        self.map_err(|e| anyhow::Error::new(e))
    }
}
