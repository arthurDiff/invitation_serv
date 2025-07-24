pub trait AsyncExt<T, E> {
    fn map_err_async<NE, NF, F: FnOnce(E) -> NF>(self, f: F) -> impl Future<Output = Result<T, NE>>
    where
        NF: Future<Output = NE>;
}

impl<T, E> AsyncExt<T, E> for Result<T, E> {
    async fn map_err_async<NE, NF, F: FnOnce(E) -> NF>(self, f: F) -> Result<T, NE>
    where
        NF: Future<Output = NE>,
    {
        match self {
            Ok(t) => Ok(t),
            Err(err) => Err(f(err).await),
        }
    }
}
