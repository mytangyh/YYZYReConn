use std::process::Command;
use std::thread;
use std::time::Duration;

// 检查防火墙规则是否存在
pub fn is_fw_rule() -> bool {
    let output = Command::new("netsh")
        .args(["advfirewall", "firewall", "show", "rule", "name=block_yyzy"])
        .output()
        .expect("执行命令失败");

    // 如果命令输出包含规则名，说明规则存在
    String::from_utf8_lossy(&output.stdout)
        .contains("block_yyzy")
}

// 修改防火墙规则
pub fn change_firewall_rule(enable: bool) -> Result<(), Box<dyn std::error::Error>> {
    let action = if enable { "add" } else { "delete" };
    
    // 执行防火墙命令
    let output = Command::new("netsh")
        .args([
            "advfirewall",
            "firewall",
            action,
            "rule",
            "name=block_yyzy",
            "dir=out",
            "action=block",
            "program=C:\\Program Files\\YYZYGame\\Game\\YYZYGame.exe",
        ])
        .output()?;

    if !output.status.success() {
        error!("防火墙规则修改失败: {}", String::from_utf8_lossy(&output.stderr));
        return Err("防火墙规则修改失败".into());
    }

    info!("防火墙规则修改成功");
    Ok(())
}

// 主程序逻辑
pub fn start() {
    info!("开始执行拔线程序");
    
    // 1. 添加防火墙规则阻断游戏
    if let Err(e) = change_firewall_rule(true) {
        error!("添加防火墙规则失败: {:?}", e);
        return;
    }

    // 2. 等待一小段时间
    thread::sleep(Duration::from_secs(1));

    // 3. 移除防火墙规则恢复连接
    if let Err(e) = change_firewall_rule(false) {
        error!("移除防火墙规则失败: {:?}", e);
        return;
    }

    info!("拔线程序执行完成");
}
