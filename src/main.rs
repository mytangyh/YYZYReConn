mod program;

use hotkey;
use std::thread;
use systray::Application;
use webbrowser;

#[macro_use]
extern crate log;
extern crate simplelog;
use simplelog::*;
use std::fs::File;

fn init_logging() -> Result<(), Box<dyn std::error::Error>> {
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Warn,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create("run.log")?,
        ),
    ])?;
    info!("日志系统初始化成功");
    Ok(())
}

fn register_hotkey() {
    info!("开始注册热键");
    thread::spawn(|| {
        let mut hk = hotkey::Listener::new();
        hk.register_hotkey(hotkey::modifiers::SHIFT | hotkey::modifiers::ALT, 'R' as u32, || {
            info!("热键被触发，执行程序");
            program::start();
        })
        .unwrap_or_else(|e| {
            error!("注册热键失败: {:?}", e);
            -1 // 或者任何合适的默认 i32 值
        });
        info!("开始监听热键");
        hk.listen();
    });
}

fn create_systray() -> Result<Application, systray::Error> {
    info!("开始创建系统托盘菜单");
    
    let mut app = Application::new()?;
    app.set_tooltip("YYZY重连工具")?;
    
    app.add_menu_item("开始拔线(Shift+Alt+R)", move |_| {
        info!("点击菜单：开始拔线");
        program::start();
        Ok::<_, systray::Error>(())
    })?;
    
    app.add_menu_separator()?;
    
    app.add_menu_item("关于我", |_| {
        info!("点击菜单：关于我");
        if let Err(e) = webbrowser::open("https://github.com/mytangyh/YYZYReConn") {
            error!("打开网页失败: {:?}", e);
        }
        Ok::<_, systray::Error>(())
    })?;
    
    app.add_menu_item("退出程序", |window| {
        info!("点击菜单：退出程序");
        window.quit();
        Ok::<_, systray::Error>(())
    })?;

    info!("系统托盘菜单创建成功");
    Ok(app)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logging()?;
    info!("程序启动");
    
    if program::is_fw_rule() {
        info!("检测到防火墙规则，正在恢复网络");
        program::change_firewall_rule(false)?;
    }

    register_hotkey();
    
    let mut app = create_systray()?;
    info!("进入系统托盘消息循环");
    app.wait_for_message()?;
    
    info!("程序正常退出");
    Ok(())
}
