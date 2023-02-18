#![allow(incomplete_features)]
#![feature(async_fn_in_trait)]

use color_eyre::Result;
use proto::packet::{Item, PlayerInventory, PreChunk, SpawnPosition};
use std::net::SocketAddr;

use monoio::{
    net::{TcpListener, TcpStream},
    spawn,
};
use tracing::{debug, info, instrument};
use tracing_subscriber::EnvFilter;

use crate::proto::{
    packet::{LoginResponse, Packet, PacketKind},
    Packets,
};

pub mod proto;

async fn fake_chunks(s: &mut TcpStream) -> Result<()> {
    PreChunk {
        mode: true,
        x: 0,
        z: 0,
    }
    .encode(s)
    .await?;

    SpawnPosition { x: 0, y: 0, z: 0 }.encode(s).await?;

    let diamond = Item {
        count: 64,
        id: 57,
        used: 0,
    };

    let iv_main = PlayerInventory {
        count: 26,
        kind: proto::packet::InventoryKind::MainInventory,
        payload: vec![diamond.clone(); 26],
    };

    let iv_armor = PlayerInventory {
        count: 4,
        kind: proto::packet::InventoryKind::EquippedArmor,
        payload: vec![diamond.clone(); 4],
    };

    let iv_craft = PlayerInventory {
        count: 4,
        kind: proto::packet::InventoryKind::CrafitingSlots,
        payload: vec![diamond; 4],
    };

    iv_main.encode(s).await?;
    iv_armor.encode(s).await?;
    iv_craft.encode(s).await?;
    Ok(())
}

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

            PacketKind::LoginRequest(req) => {
                tracing::info!("Player <{}> is logging...", req.username);
                let response = LoginResponse {
                    server_name: "".to_string(),
                    server_motd: "".to_string(),
                    map_seed: 971768181197178410,
                    dimension: -1,
                    entity_id: 0,
                };

                response.encode(&mut stream).await?;
                debug!("Sent login response");
                fake_chunks(&mut stream).await?;
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
