use smol::{channel::Sender, io::AsyncReadExt, net, stream::StreamExt};
use way_quick::Event;

pub async fn run(tx: Sender<Event>) -> anyhow::Result<()> {
    #[cfg(target_os = "linux")]
    {
        let listener = net::TcpListener::bind("127.0.0.1:4343").await?;
        let mut buffer = [0; 1024];
        while let Some(stream) = listener.incoming().next().await {
            let mut stream = stream?;
            let bytes_read = stream.read(&mut buffer).await?;
            let _received = String::from_utf8_lossy(&buffer[..bytes_read]);
            tx.send(Event::Launcher).await?;
        }
    }
    #[cfg(target_os = "windows")]
    {
        tx.send(Event::Launcher).await?;
    }
    Ok(())
}
