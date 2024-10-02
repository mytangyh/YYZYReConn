use std::os::windows::process::CommandExt;
use std::process::Command;
use std::{thread, time};
use textcode::gb2312;

const CREATE_NO_WINDOW: u32 = 0x08000000;
const FIREWALL_RULE_NAME: &str = "Rule_YYZY";

fn execute_command(cmd: &mut Command) -> Result<String, String> {
    cmd.creation_flags(CREATE_NO_WINDOW);
    let output = cmd.output().map_err(|e| format!("命令执行失败: {:?}", e))?;

    let mut decoded_output = String::new();
    gb2312::decode(&output.stdout, &mut decoded_output);
    Ok(decoded_output.trim().to_string())
}

fn get_yyzy_path() -> Result<String, String> {
    info!("开始获取月圆之夜路径");

    let output = execute_command(
        Command::new("wmic")
            .arg("Process")
            .arg("where")
            .arg("name='Night of the Full Moon.exe'")
            .arg("get")
            .arg("executablepath"),
    )?;

    let path: Vec<&str> = output.split_whitespace().collect();
    if path.len() > 1 {
        let yyzy_path = path[1].to_string();
        info!("月圆之夜路径获取成功: {}", yyzy_path);
        Ok(yyzy_path)
    } else {
        Err("获取月圆之夜路径失败".to_string())
    }
}

pub fn is_fw_rule() -> bool {
    info!("开始判断防火墙规则是否存在");

    match execute_command(
        Command::new("netsh")
            .arg("advfirewall")
            .arg("firewall")
            .arg("show")
            .arg("rule")
            .arg(format!("name={}", FIREWALL_RULE_NAME)),
    ) {
        Ok(output) => {
            let rule_exists = output.contains(FIREWALL_RULE_NAME);
            info!("防火墙规则是否存在: {}", rule_exists);
            rule_exists
        }
        Err(e) => {
            error!("判断防火墙规则失败: {}", e);
            false
        }
    }
}

fn create_firewall_rule(yyzy_path: &str) -> Result<(), String> {
    info!("开始创建防火墙规则");

    execute_command(
        Command::new("netsh")
            .arg("advfirewall")
            .arg("firewall")
            .arg("add")
            .arg("rule")
            .arg(format!("name={}", FIREWALL_RULE_NAME))
            .arg("dir=out")
            .arg("action=block")
            .arg(format!("program={}", yyzy_path))
            .arg("enable=NO"),
    )?;

    info!("防火墙规则创建成功");
    Ok(())
}

pub fn change_firewall_rule(enable: bool) -> Result<(), String> {
    let action = if enable { "启用" } else { "禁用" };
    info!("开始{}防火墙规则", action);

    execute_command(
        Command::new("netsh")
            .arg("advfirewall")
            .arg("firewall")
            .arg("set")
            .arg("rule")
            .arg(format!("name={}", FIREWALL_RULE_NAME))
            .arg("new")
            .arg(format!("enable={}", if enable { "YES" } else { "NO" })),
    )?;

    info!("{}防火墙规则结束", action);
    Ok(())
}

fn start_reconnection() -> Result<(), String> {
    info!("月圆之夜重连开始");
    change_firewall_rule(true)?;
    thread::sleep(time::Duration::from_millis(3000));
    change_firewall_rule(false)?;
    info!("月圆之夜重连结束");
    Ok(())
}

pub fn start() {
    if !is_fw_rule() {
        match get_yyzy_path() {
            Ok(yyzy_path) => {
                if let Err(e) = create_firewall_rule(&yyzy_path) {
                    error!("创建防火墙规则失败: {}", e);
                    return;
                }
            }
            Err(e) => {
                error!("获取月圆之夜路径失败: {}", e);
                return;
            }
        }
    }

    if let Err(e) = start_reconnection() {
        error!("重连失败: {}", e);
    }
}
