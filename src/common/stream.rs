use quinn::{RecvStream as QuinnRecvStream, SendStream as QuinnSendStream};
use std::{
    io::{IoSlice, Result},
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};

pub(crate) type StreamReg = Arc<()>;

#[derive(Debug)]
pub struct SendStream(QuinnSendStream, StreamReg);

impl SendStream {
    #[inline]
    pub(crate) fn new(send: QuinnSendStream, reg: StreamReg) -> Self {
        Self(send, reg)
    }

    #[inline]
    pub async fn finish(&mut self) -> Result<()> {
        self.0.finish().await?;
        Ok(())
    }
}

impl AsyncWrite for SendStream {
    #[inline]
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize>> {
        Pin::new(&mut self.0).poll_write(cx, buf)
    }

    #[inline]
    fn poll_write_vectored(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &[IoSlice<'_>],
    ) -> Poll<Result<usize>> {
        Pin::new(&mut self.0).poll_write_vectored(cx, bufs)
    }

    #[inline]
    fn is_write_vectored(&self) -> bool {
        self.0.is_write_vectored()
    }

    #[inline]
    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
        Pin::new(&mut self.0).poll_flush(cx)
    }

    #[inline]
    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
        Pin::new(&mut self.0).poll_shutdown(cx)
    }
}

#[derive(Debug)]
pub struct RecvStream(QuinnRecvStream, StreamReg);

impl RecvStream {
    #[inline]
    pub(crate) fn new(recv: QuinnRecvStream, reg: StreamReg) -> Self {
        Self(recv, reg)
    }
}

impl AsyncRead for RecvStream {
    #[inline]
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<Result<()>> {
        Pin::new(&mut self.0).poll_read(cx, buf)
    }
}

#[derive(Debug)]
pub struct BiStream(SendStream, RecvStream);

impl BiStream {
    #[inline]
    pub(crate) fn new(send: SendStream, recv: RecvStream) -> Self {
        Self(send, recv)
    }

    #[inline]
    pub async fn finish(&mut self) -> Result<()> {
        self.0.finish().await
    }
}

impl AsyncWrite for BiStream {
    #[inline]
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize>> {
        Pin::new(&mut self.0).poll_write(cx, buf)
    }

    #[inline]
    fn poll_write_vectored(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &[IoSlice<'_>],
    ) -> Poll<Result<usize>> {
        Pin::new(&mut self.0).poll_write_vectored(cx, bufs)
    }

    #[inline]
    fn is_write_vectored(&self) -> bool {
        self.0.is_write_vectored()
    }

    #[inline]
    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
        Pin::new(&mut self.0).poll_flush(cx)
    }

    #[inline]
    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
        Pin::new(&mut self.0).poll_shutdown(cx)
    }
}

impl AsyncRead for BiStream {
    #[inline]
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<Result<()>> {
        Pin::new(&mut self.1).poll_read(cx, buf)
    }
}