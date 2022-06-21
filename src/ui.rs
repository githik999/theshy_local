extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use lucian::server::Server;
use nwd::NwgUi;
use nwg::NativeUi;


#[derive(Default, NwgUi)]
pub struct SystemTray {
    #[nwg_control]
    window: nwg::MessageWindow,

    #[nwg_resource(source_file: Some("local.ico"))]
    icon: nwg::Icon,

    #[nwg_control(icon: Some(&data.icon), tip: Some("theshy local"))]
    #[nwg_events(MousePressLeftUp: [SystemTray::show_menu], OnContextMenu: [SystemTray::show_menu])]
    tray: nwg::TrayNotification,

    #[nwg_control(parent: window, popup: true)]
    tray_menu: nwg::Menu,

    #[nwg_control(parent: tray_menu, text: "status")]
    #[nwg_events(OnMenuItemSelected: [SystemTray::check_status])]
    tray_item2: nwg::MenuItem,

    #[nwg_control(parent: tray_menu, text: "exit")]
    #[nwg_events(OnMenuItemSelected: [SystemTray::exit])]
    tray_item3: nwg::MenuItem,
}

impl SystemTray {

    fn show_menu(&self) {
        let (x, y) = nwg::GlobalCursor::position();
        self.tray_menu.popup(x, y);
    }
    
    fn check_status(&self) {
        let status = Server::status();
        let title = format!("{:?}",status);
        let flags = nwg::TrayNotificationFlags::USER_ICON | nwg::TrayNotificationFlags::LARGE_ICON;
        self.tray.show("visit 127.0.0.1:1081 for more details", Some(title.as_str()), Some(flags), Some(&self.icon));
    }
    
    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }

}

pub fn start() {
    nwg::init().expect("Failed to init Native Windows GUI");
    let icon = SystemTray::build_ui(Default::default()).expect("Failed to build UI");
    icon.check_status();
    nwg::dispatch_thread_events();
}
