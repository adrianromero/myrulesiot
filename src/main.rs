use tokio::{task, time};

use rumqttc::{self, AsyncClient, MqttOptions, QoS};
use std::error::Error;
use std::time::Duration;

mod mqtt;
use mqtt::ConnectionInfo;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    log::trace!("Executing query: {}", "pepe");
    log::debug!("Executing query: {}", "debug");
    log::info!("Executing info.");
    log::warn!("Executing warn.");
    log::error!("Executing error.");

    // Defines connection properties
    let connection_info = ConnectionInfo {
        id: "rustclient-231483".into(),
        host: "localhost".into(),
        clean_session: true,
        ..Default::default()
    };

    connectivity(connection_info).await?;

    Ok(())
}

async fn connectivity(connection_info: ConnectionInfo) -> Result<(), Box<dyn Error>> {
    // Connects to the MQTT server
    let mut mqttoptions = MqttOptions::new(
        connection_info.id,
        connection_info.host,
        connection_info.port,
    );
    mqttoptions
        .set_keep_alive(connection_info.keep_alive)
        .set_inflight(connection_info.inflight)
        .set_clean_session(connection_info.clean_session);

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    client
        .subscribe("myhelloiot/modal", QoS::AtMostOnce)
        .await?;

    task::spawn(async move {
        requests(client).await;
        time::sleep(Duration::from_secs(3)).await;
    });

    // See example in https://github.com/bytebeamio/rumqtt/issues/263
    loop {
        let result = eventloop.poll().await;
        match result {
            Result::Ok(event) => {
                println!("Event -> {:?}", event);
            }
            Result::Err(error) => {
                println!("Error -> {:?}", error);
                Result::Err(error)?;
            }
        }
    }
}

async fn requests(client: AsyncClient) {
    // for i in 1..=10 {
    //     client
    //         .publish("hello/world", QoS::ExactlyOnce, false, vec![1; i])
    //         .await
    //         .unwrap();

    //     time::sleep(Duration::from_secs(1)).await;
    // }

    time::sleep(Duration::from_secs(10)).await;
    client.cancel().await.unwrap();
    //client.disconnect().await.unwrap();
}
