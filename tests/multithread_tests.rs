use std::{net::SocketAddr, sync::Arc, time::Duration};
use tokio::sync::Mutex;
use vmix_rs::vmix::VmixApi;

#[cfg(test)]
mod tests {
    use super::*;
    use quick_xml::de;
    use vmix_rs::models::{Input, State};
    use vmix_rs::traits::VmixTcpApiClient;

    #[test]
    fn test_actual_vmix_xml_parsing() {
        // Test with actual vMix XML from http://192.168.160.54:8088/api
        let actual_input_running = r#"<input key="f9bc0129-670f-440c-8ef4-fbf7173dfbda" number="3" type="Capture" title="CAMSWer" shortTitle="CAMSWer" state="Running" position="0" duration="0" loop="False" muted="True" volume="100" balance="0" solo="False" soloPFL="False" audiobusses="M" meterF1="0" meterF2="0" gainDb="0">CAMSWer</input>"#;

        let actual_input_paused = r#"<input key="f7bf0353-adfc-4087-abd6-47a121b54016" number="1" type="Replay" title="20250814_STAGE0 - A" shortTitle="20250814_STAGE0" state="Paused" position="8296889" duration="8296972" loop="True" muted="False" volume="100" balance="0" solo="False" soloPFL="False" audiobusses="M" meterF1="0" meterF2="0" gainDb="0">20250814_STAGE0 - A</input>"#;

        // Test parsing
        let running_result: Result<Input, _> = de::from_str(actual_input_running);
        let paused_result: Result<Input, _> = de::from_str(actual_input_paused);

        match running_result {
            Ok(input) => {
                assert_eq!(input.state, State::Running);
                println!("✅ Running state parsed correctly: {:?}", input.state);
            }
            Err(e) => {
                println!("❌ Failed to parse Running state: {}", e);
                panic!("Running state parsing failed");
            }
        }

        match paused_result {
            Ok(input) => {
                assert_eq!(input.state, State::Paused);
                println!("✅ Paused state parsed correctly: {:?}", input.state);
            }
            Err(e) => {
                println!("❌ Failed to parse Paused state: {}", e);
                panic!("Paused state parsing failed");
            }
        }
    }

    #[tokio::test]
    #[cfg(feature = "http")]
    async fn test_actual_vmix_xml_full_parsing() {
        // Test if we can fetch and parse the actual XML
        use std::time::Duration;
        use vmix_rs::http::HttpVmixClient;

        let client =
            HttpVmixClient::new_with_host_port("192.168.160.54", 8088, Duration::from_secs(5));

        match client.get_xml_state().await {
            Ok(vmix_data) => {
                println!("✅ Successfully parsed actual vMix XML");
                println!("Total inputs: {}", vmix_data.inputs.input.len());

                for input in &vmix_data.inputs.input {
                    println!(
                        "Input {}: {} - state: {:?}",
                        input.number, input.title, input.state
                    );
                }
            }
            Err(e) => {
                println!(
                    "⚠️ Could not connect to vMix instance at 192.168.160.54:8088: {}",
                    e
                );
            }
        }
    }

    #[tokio::test]
    async fn test_vmix_api_send_sync_traits() {
        // This test will only compile if VmixApi implements Send + Sync
        fn assert_send<T: Send>() {}
        fn assert_sync<T: Sync>() {}

        assert_send::<VmixApi>();
        assert_sync::<VmixApi>();
    }

    #[tokio::test]
    async fn test_vmix_api_with_arc_mutex() {
        // Test that VmixApi can be used with Arc<Mutex<>>
        // This simulates the TCP-only usage pattern

        // Note: This test uses a dummy address that won't connect
        // In a real scenario, you would use a valid vMix instance
        let addr: SocketAddr = "127.0.0.1:8099".parse().unwrap();
        let timeout = Duration::from_secs(5);

        // Test Arc<tokio::sync::Mutex<VmixApi>>
        let result = VmixApi::new(addr, timeout);

        // Since we're not running an actual vMix instance, the connection will fail
        // But this test ensures that the type system accepts our Send + Sync implementation
        match result {
            Ok(api) => {
                let api_arc = Arc::new(Mutex::new(api));
                let api_clone = api_arc.clone();

                // This should compile without errors if Send + Sync are properly implemented
                tokio::spawn(async move {
                    let client = api_clone.lock().await;
                    // Test TCP-specific methods
                    let _connected = client.is_connected();
                    // In a real test with a running vMix instance:
                    // let _result = client.send_command(SendCommand::TALLY);
                })
                .await
                .unwrap();
            }
            Err(_) => {
                // Connection failed as expected since no vMix instance is running
                // But the important thing is that the code compiles
            }
        }
    }

    #[tokio::test]
    async fn test_vmix_api_with_std_mutex() {
        // Test that VmixApi can be used with Arc<std::sync::Mutex<>>
        let addr: SocketAddr = "127.0.0.1:8099".parse().unwrap();
        let timeout = Duration::from_secs(5);

        let result = VmixApi::new(addr, timeout);

        match result {
            Ok(api) => {
                let api_arc = Arc::new(std::sync::Mutex::new(api));
                let api_clone = api_arc.clone();

                // This should compile without errors if Send + Sync are properly implemented
                tokio::spawn(async move {
                    let client = api_clone.lock().unwrap();
                    // Test TCP-specific methods (non-async)
                    let _connected = client.is_connected();
                })
                .await
                .unwrap();
            }
            Err(_) => {
                // Connection failed as expected
            }
        }
    }

    #[tokio::test]
    async fn test_spawn_multiple_tasks_tcp_only() {
        // Test spawning multiple tasks that would use VmixApi for TCP operations
        let addr: SocketAddr = "127.0.0.1:8099".parse().unwrap();
        let timeout = Duration::from_secs(1);

        let result = VmixApi::new(addr, timeout);

        match result {
            Ok(api) => {
                let api_arc = Arc::new(Mutex::new(api));

                let handles: Vec<_> = (0..3)
                    .map(|_| {
                        let api_clone = api_arc.clone();
                        tokio::spawn(async move {
                            let client = api_clone.lock().await;
                            // Test TCP-specific operations
                            let _connected = client.is_connected();
                            // In a real scenario with a running vMix instance:
                            // let sender = client.get_sender();
                            // let _result = client.try_receive_command(Duration::from_millis(100));
                        })
                    })
                    .collect();

                // Wait for all tasks to complete
                for handle in handles {
                    handle.await.unwrap();
                }
            }
            Err(_) => {
                // Connection failed as expected
            }
        }
    }

    #[test]
    fn test_tcp_api_traits() {
        // Compile-time test to ensure VmixApi implements VmixTcpApiClient
        fn assert_tcp_api_client<T: VmixTcpApiClient>() {}
        assert_tcp_api_client::<VmixApi>();
    }

    #[tokio::test]
    async fn test_shutdown_handling() {
        // Test that VmixApi shuts down properly even when connection fails
        let addr: SocketAddr = "127.0.0.1:8099".parse().unwrap();
        let timeout = Duration::from_millis(100); // Short timeout for quick test

        let result = VmixApi::new(addr, timeout);

        match result {
            Ok(api) => {
                // If connection succeeds, test normal shutdown
                assert!(api.is_connected());
                drop(api); // This should trigger clean shutdown
                // If we reach here, shutdown didn't hang
            }
            Err(_) => {
                // Expected - no vMix instance running
                // This is fine for testing shutdown behavior
            }
        }
    }
}
