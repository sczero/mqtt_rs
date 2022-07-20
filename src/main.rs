use std::error::Error;
use std::string::FromUtf8Error;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use paho_mqtt as mqtt;
use paho_mqtt::{ConnectOptions, ConnectOptionsBuilder, Message, SslOptions, SslOptionsBuilder};
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let handle1 = tokio::spawn(async move {
        //监听
        let client = Arc::new(mqtt::AsyncClient::new("tcp://broker-cn.emqx.io:1883").unwrap());
        let conn_options = ConnectOptionsBuilder::new()
            .clean_session(true)//测试环境必须是true
            .automatic_reconnect(Duration::from_secs(5), Duration::from_secs(5))
            .user_name("11")
            .password("222")
            .finalize();
        client.connect(conn_options).await.unwrap();

        client.subscribe("app_up/#", 1).await.unwrap();
        for item in client.start_consuming().iter() {
            println!("新消息");
            if let Some(msg) = item {
                match String::from_utf8(msg.payload().to_vec()) {
                    Ok(msg) => {
                        println!("监听消息:{}", msg);
                    }
                    Err(_) => {}
                }
            }
        }
        client.disconnect(None).await.unwrap();
    });
    let handle2 = tokio::spawn(timer());
    tokio::join!(handle1,handle2);
    Ok(())
}

async fn timer() {
    loop {
        println!("tick");
        let _ = sleep(Duration::from_secs(5)).await;
    };
}

async fn mqtt_subscribe() {}
