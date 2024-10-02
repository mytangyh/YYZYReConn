#![windows_subsystem = "windows"]
mod firewall;

use hotkey;
use std::thread;
use systray;
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
    Ok(())
}

fn register_hotkey() {
    thread::spawn(|| {
        let mut hk = hotkey::Listener::new();
        hk.register_hotkey(hotkey::modifiers::SHIFT | hotkey::modifiers::ALT, 'R' as u32, || {
            program::start();
        })
        .unwrap_or_else(|e| error!("注册热键失败: {:?}", e));
        hk.listen();
    });
}

fn create_systray() -> Result<systray::Application, systray::Error> {
    let mut app = systray::Application::new()?;
    let icon = include_bytes!("./assets/yyzy.png");
    app.set_icon_from_buffer(icon, 64, 64)?;
    
    app.add_menu_item("开始拔线(Shift+Alt+R)", move |_| {
        program::start();
        Ok::<_, systray::Error>(())
    })?;
    
    app.add_menu_separator()?;
    
    app.add_menu_item("关于我", |_| {
        if let Err(e) = webbrowser::open("https://blog.3gxk.net/about.html") {
            error!("打开网页失败: {:?}", e);
        }
        Ok::<_, systray::Error>(())
    })?;
    
    app.add_menu_item("退出程序", |window| {
        window.quit();
        Ok::<_, systray::Error>(())
    })?;

    Ok(app)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logging()?;

    if program::is_fw_rule() {
        // 恢复网络
        program::disable();
    }

    register_hotkey();

    let mut app = create_systray()?;
    app.wait_for_message()?;
    
    Ok(())
}
