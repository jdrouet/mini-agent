#![allow(async_fn_in_trait)]

pub trait Component: Sized {
    async fn run(self);
}
