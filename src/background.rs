use lucian::gate::hub::line_header::LineType;
use lucian::log::Log;
use lucian::server::Server;

const CALLER_NUM: u8 = 16;
const APP_ADDR:&str = "0.0.0.0:1080";
const LOG_ADDR:&str = "0.0.0.0:1081";
const PROXY_SERVER:&str = "8.218.15.102:3389";

pub fn start() {
    Log::init();
    let mut app = Server::new(APP_ADDR,LineType::Fox);
    app.init(CALLER_NUM,PROXY_SERVER);
    let mut http = Server::new(LOG_ADDR,LineType::Http);
    std::thread::spawn(move ||{
        http.start();
    });
    app.start();
}