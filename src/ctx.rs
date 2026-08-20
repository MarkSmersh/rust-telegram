// use std::{pin::Pin, sync::Arc};
//
// pub type Handler<Ctx, Args> = dyn Fn(Arc<Ctx>, Args) -> BoxFuture + Send + Sync;
// pub type BoxFuture = Pin<Box<dyn Future<Output = ()>>>;
//
// pub struct CtxFn<Ctx, Args> {
//     ctx: Arc<Ctx>,
//     f: Arc<Handler<Ctx, Args>>,
// }
//
// impl<Ctx, Args> Clone for CtxFn<Ctx, Args> {
//     fn clone(&self) -> Self {
//         Self {
//             ctx: Arc::clone(&self.ctx),
//             f: Arc::clone(&self.f),
//         }
//     }
// }
//
// impl<Ctx, Args> CtxFn<Ctx, Args> {
//     pub fn new<F>(ctx: Ctx, f: F) -> Self
//     where
//         F: Fn(Arc<Ctx>, Args) -> BoxFuture + 'static + Sync + Send,
//     {
//         Self {
//             ctx: Arc::new(ctx),
//             f: Arc::new(f),
//         }
//     }
//
//     pub async fn exec(&mut self, args: Args) {
//         let _ = (self.f)(self.ctx.clone(), args).await;
//     }
// }
