use mdns_sd::{ServiceDaemon, ServiceEvent, ServiceInfo};
use std::collections::HashMap;
use std::time::Duration;

// サービスタイプを検索し、そのサービスの詳細情報を取得する
pub async fn resolve_services(service_type: &str, search_duration: u64) -> HashMap<String, (String, ServiceInfo)> {
    // 検出されたサービスを保存するためのハッシュマップ
    let mut discovered_services = HashMap::new();
    
    // mDNSデーモンを作成
    let mdns = ServiceDaemon::new().expect("mDNSデーモンの作成に失敗しました");
    
    // ブラウザを作成して、指定されたサービスを監視
    let receiver = mdns.browse(service_type).expect("ブラウザの作成に失敗しました");
    
    // 検索開始時間
    let start_time = std::time::Instant::now();
    
    // 指定された秒数間検索を実行
    while start_time.elapsed() < Duration::from_secs(search_duration) {
        if let Ok(event) = receiver.recv_timeout(Duration::from_millis(100)) {
            match event {
                ServiceEvent::ServiceResolved(info) => {
                    // 解決されたサービス情報を保存
                    discovered_services.insert(info.get_fullname().to_string(), (service_type.to_string(), info.clone()));
                },
                ServiceEvent::ServiceRemoved(_, fullname) => {
                    // 削除されたサービスをハッシュマップから削除
                    discovered_services.remove(&fullname);
                },
                _ => {}
            }
        }
    }
    
    println!("サービスタイプ '{}' の検索を終了（{}秒経過）", service_type, search_duration);
    
    // 検出されたサービスのハッシュマップを返す
    discovered_services
}

pub async fn discover_and_resolve_services(search_duration: u64, verbose: bool, service_type: Option<&str>) {
    let mdns = ServiceDaemon::new().expect("mDNSデーモンの作成に失敗しました");
    let browse_type = service_type.unwrap_or("_services._dns-sd._udp.local.");
    let receiver = mdns.browse(browse_type).expect("ブラウザの作成に失敗しました");

    println!("利用可能なサービスタイプを検索中...");
    let start_time = std::time::Instant::now();

    let mut all_services = HashMap::new();
    let mut handles = vec![];

    if service_type.is_none() {
        // サービスタイプが指定されていない場合、すべてのサービスを解決
        while start_time.elapsed() < Duration::from_secs(search_duration) {
            if let Ok(event) = receiver.recv_timeout(Duration::from_millis(100)) {
                match event {
                    ServiceEvent::ServiceFound(_, fullname) => {
                        println!("サービスタイプを発見: {}", &fullname);
                        let handle = tokio::spawn(async move {
                            resolve_services(&fullname, search_duration).await
                        });
                        handles.push(handle);
                    },
                    _ => {}
                }
            }
        }
    } else {
        // サービスタイプが指定されている場合、そのサービスのみを解決
        let services = resolve_services(browse_type, search_duration).await;
        all_services.extend(services);
    }

    for handle in handles {
        if let Ok(services) = handle.await {
            all_services.extend(services);
        }
    }

    println!("\n===== 検索結果のまとめ =====");
    println!("検出されたサービス数: {}", all_services.len());

    if all_services.is_empty() {
        println!("サービスは検出されませんでした。");
    } else {
        let mut services_by_type: HashMap<String, Vec<(String, ServiceInfo)>> = HashMap::new();
        for (fullname, (service_type, info)) in all_services {
            services_by_type.entry(service_type).or_insert_with(Vec::new).push((fullname, info));
        }

        for (service_type, services) in services_by_type {
            println!("\nサービスタイプ: {}", service_type);
            for (fullname, info) in services {
                if verbose {
                    println!("  サービス: {}", fullname);
                    println!("    ホスト名: {}", info.get_hostname());
                    println!("    アドレス: {:?}", info.get_addresses());
                    println!("    ポート: {}", info.get_port());
                    println!("    TXT レコード: {:?}", info.get_properties());
                } else {
                    println!("  サービス: {}", fullname);
                    println!("    ホスト名: {}", info.get_hostname());
                    if let Some(address) = info.get_addresses().iter().next() {
                        println!("    アドレス: {}:{}", address, info.get_port());
                    }
                }
            }
        }
    }

    println!("===== 検索完了 =====");
}
