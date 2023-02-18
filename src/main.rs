#![allow(incomplete_features)]
#![feature(async_fn_in_trait)]

use color_eyre::Result;
use std::net::SocketAddr;

use monoio::{
    net::{TcpListener, TcpStream},
    spawn,
};
use tracing::{info, instrument};
use tracing_subscriber::EnvFilter;

use crate::proto::{
    packet::{Packet, PacketKind},
    Packets,
};

pub mod proto;

#[instrument(name = "Handle client", skip(stream))]
async fn accept(mut stream: TcpStream, addr: SocketAddr) -> Result<()> {
    tracing::info!("accepted connection");
    loop {
        let packet = Packets::decode(&mut stream).await?;

        match packet {
            PacketKind::Handshake(mut hshake) => {
                tracing::info!("Player <{}> is trying to join the server", hshake.nick);

                hshake.nick = "-".to_string();
                hshake.encode(&mut stream).await?;
            }
        }
    }
}

#[monoio::main]
async fn main() -> Result<()> {
    let _ = color_eyre::install();

    tracing_subscriber::FmtSubscriber::builder()
        .pretty()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive("turboalpha=info".parse().unwrap())
                .from_env_lossy(),
        )
        .init();

    let listener = TcpListener::bind("0.0.0.0:9898")?;
    info!("listening on :9898");

    loop {
        let (stream, addr) = listener.accept().await?;
        spawn(async move {
            if let Err(e) = accept(stream, addr).await {
                tracing::warn!(err = ?e, "failed to handle!");
            }
        });
    }
}
