// Example for impl AsyncWrite to own trait
use std::io::Write;
use std::str;
use tokio::io::AsyncWrite;
use tokio::io::AsyncWriteExt;

use bytes::Bytes;

struct StdoutWriter;

impl AsyncWrite for StdoutWriter {
    fn poll_write(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<std::io::Result<usize>> {
        // write stdio from buffer
        let stdout = std::io::stdout();
        let mut handle = stdout.lock();
        match handle.write(buf) {
            Ok(n) => std::task::Poll::Ready(Ok(n)),
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                cx.waker().wake_by_ref();
                std::task::Poll::Pending
            }
            Err(e) => std::task::Poll::Ready(Err(e)),
        }
    }

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        // flush the stdio
        let stdout = std::io::stdout();
        let mut handle = stdout.lock();
        match handle.flush() {
            Ok(()) => std::task::Poll::Ready(Ok(())),
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                cx.waker().wake_by_ref();
                std::task::Poll::Pending
            }
            Err(e) => std::task::Poll::Ready(Err(e)),
        }
    }

    fn poll_shutdown(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        // Do nothing
        std::task::Poll::Ready(Ok(()))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, mut rx) = tokio::sync::mpsc::channel(10);

    let mut stdout = StdoutWriter {};

    tx.send(Bytes::from("Hello")).await?;
    tx.send(Bytes::from("World")).await?;

    let s1 = rx.recv().await.unwrap();
    let s2 = rx.recv().await.unwrap();

    print!(
        "{} {}",
        str::from_utf8(&s1).unwrap(),
        str::from_utf8(&s2).unwrap()
    );

    stdout.write_u8(b'!').await?;
    stdout.flush().await?;

    Ok(())
}
