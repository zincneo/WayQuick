use macro_rules_attribute::apply;
use smol::channel::unbounded;
use smol_macros::{Executor, main};
use way_quick::Event;

mod app;

mod web_server;

#[apply(main!)]
async fn main(ex: &Executor<'_>) {
    let (tx, rx) = unbounded::<Event>();
    let web_server_task = ex.spawn(web_server::run(tx));
    let app_task = ex.spawn(app::run(rx));

    app_task.await;

    println!("{:?}", web_server_task.cancel().await);
}
