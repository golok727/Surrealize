use core::fmt;

use super::repository::Repository;

use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Entry<'a, T>
where
    T: Serialize + ?Sized + for<'de> Deserialize<'de> + 'static,
{
    data: T,
    repo: &'a Repository<T>,
}

impl<'a, T> Entry<'a, T>
where
    T: Serialize + ?Sized + for<'de> Deserialize<'de> + 'static,
{
    pub(crate) fn new(data: T, repo: &'a Repository<T>) -> Self {
        Self { data, repo }
    }
    pub fn data_mut(&self) -> &T {
        &self.data
    }
    pub fn data(&self) -> &T {
        &self.data
    }
    pub fn data_clone(&self) -> T
    where
        T: Clone,
    {
        self.data.clone()
    }

    pub fn delete(&self) {
        let _repo = self.repo;
        todo!()
    }
}

impl<'a, T> fmt::Display for Entry<'a, T>
where
    T: Serialize + ?Sized + for<'de> Deserialize<'de> + 'static + std::fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self.data)
    }
}

impl<'a, T> Serialize for Entry<'a, T>
where
    T: Serialize + ?Sized + for<'de> Deserialize<'de> + 'static,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.data.serialize(serializer)
    }
}
