#[cfg(test)]
mod tests {
    use dispenser_api::bar::Bar;
    use dispenser_api::bar::models::Dispenser;

    #[tokio::test]
    async fn add_dispenser() {
        let dispenser = Dispenser{ id:None, public_key: String::from("pk"), flow_volume: 0.5};
        let bar = Bar::new();
        let new_dispenser = bar.add_dispenser(dispenser).await;
        let flow_volume = bar.get_flow_volume(new_dispenser.id).await;
        assert_eq!(flow_volume, 0.5);
    }
}

