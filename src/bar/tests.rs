mod tests {
    use crate::bar::RedisOracle;
    use crate::bar::models::Dispenser;
    use crate::bar::Bar;

    #[tokio::test]
    async fn redis_send_dispenser() {
        let dispenser = Dispenser{ public_key: String::from("pk"), flow_volume: 0.5, _id: None};
        let redis = RedisOracle::new();
        let test = redis.send(dispenser.public_key.to_owned(), dispenser.flow_volume).await;
        assert!(test);
        let flow_volume = redis.get_flow_volume(dispenser.public_key.to_owned()).await;
        assert_eq!(flow_volume, 0.5);
    }

    #[tokio::test]
    async fn bar_add_dispenser() {
        let dispenser = Dispenser{ public_key: String::from("pk"), flow_volume: 0.5, _id: None};
        let bar = Bar::new().await;
        let new_dispenser_id = bar.add_dispenser(&dispenser).await;
        assert!(new_dispenser_id.is_some());
        let flow_volume = bar.get_flow_volume(new_dispenser_id.unwrap()).await;
        assert!(flow_volume.is_some());
        assert_eq!(flow_volume.unwrap(), 0.5);
    }
}