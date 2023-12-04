pub(crate) trait Component: Sized {
    async fn run(self);
}
