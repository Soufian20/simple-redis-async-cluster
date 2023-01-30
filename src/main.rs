use redis::{
    cluster_async::{Client as ClusterClient, Connection as ClusterConnection},
    ConnectionAddr, ConnectionInfo, RedisConnectionInfo,
};

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let redis_nodes = vec![
        "redis://127.0.0.1:6370",
        "redis://127.0.0.1:6371",
        "redis://127.0.0.1:6372",
        "redis://127.0.0.1:6373",
        "redis://127.0.0.1:6374",
        "redis://127.0.0.1:6375",
    ];

    let mut connection_infos = Vec::new();
    for redis_node in redis_nodes.iter() {
        let ip_port_pair: Vec<&str> = redis_node.split(":").collect();
        let ip = ip_port_pair[0];
        let port = ip_port_pair[1].parse::<u16>().unwrap_or(6379);
        let redis_connection_info = ConnectionInfo {
            addr: ConnectionAddr::Tcp(ip.to_string(), port),
            redis: RedisConnectionInfo {
                db: 0,
                username: Some("default".to_string()),
                password: None,
            },
        };
        connection_infos.push(redis_connection_info);
    }

    let client = ClusterClient::open(redis_nodes).expect("Unable to connect to Cluster");

    let result: String = redis::cmd("PING")
        .query_async(&mut client.get_connection().await.unwrap())
        .await
        .unwrap();

    assert_eq!(result, "PONG");
    println!("Works");
}
