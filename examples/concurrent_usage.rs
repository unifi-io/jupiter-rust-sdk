use jupiter_sdk::{JupiterClient, JupiterConfig, UltraClient, models::order::OrderReq};
use std::sync::Arc;
use tokio::task;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建客户端
    let config = JupiterConfig::default();
    let jupiter_client = JupiterClient::new(config)?;
    let ultra_client = UltraClient::new(jupiter_client);
    
    // 将客户端包装在 Arc 中以便多线程共享
    let client = Arc::new(ultra_client);
    
    // 创建多个并发任务
    let mut handles = vec![];
    
    for i in 0..5 {
        let client_clone = Arc::clone(&client);
        
        let handle = task::spawn(async move {
            // 模拟不同的交易请求
            let req = OrderReq {
                input_mint: "6p6xgHyF7AeE6TZkSmFsko444wqoP15icUSqi2jfGiPN".to_string(),
                output_mint: "So11111111111111111111111111111111111111112".to_string(),
                amount: format!("{}", 100 + i * 50), // 不同的金额
                taker: "9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM".to_string(),
                referral_account: None,
                referral_fee: None,
                exclude_dexes: None,
                exclude_routers: None,
            };
            
            println!("任务 {} 开始执行", i);
            
            // 这里可以调用 ultra_client.execute() 方法
            // let result = client_clone.execute(&transaction_req).await;
            
            println!("任务 {} 完成", i);
            i
        });
        
        handles.push(handle);
    }
    
    // 等待所有任务完成
    for handle in handles {
        let result = handle.await?;
        println!("任务 {} 返回结果: {}", result, result);
    }
    
    println!("所有并发任务完成！");
    Ok(())
}
