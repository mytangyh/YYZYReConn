extern crate winres;
use std::path::Path;

fn main() {
  if cfg!(target_os = "windows") {
    let mut res = winres::WindowsResource::new();
    res.set_icon("./src/assets/ico.ico");
    res.set_manifest(
      r#"
    <assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
    <trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
        <security>
            <requestedPrivileges>
                <requestedExecutionLevel level="requireAdministrator" uiAccess="false" />
            </requestedPrivileges>
        </security>
    </trustInfo>
    </assembly>
    "#,
    );
    res.compile().unwrap();
  }

  // 确保图标文件存在
  let icon_path = Path::new("./src/assets/ico.ico");
  if !icon_path.exists() {
    panic!("Icon file not found at: {:?}", icon_path);
  }
}