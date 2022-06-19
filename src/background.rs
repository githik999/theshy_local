use lucian::gate::hub::line_header::LineType;
use lucian::log::Log;
use lucian::server::Server;

const CALLER_NUM: u8 = 30;
const APP_PORT:usize = 1080;
const LOG_PORT:usize = 1081;
const PROXY_SERVER:&str = "127.0.0.1:3389";

pub fn start() {
    Log::init();
    let mut app = Server::new(APP_PORT,LineType::Fox);
    app.init(CALLER_NUM,PROXY_SERVER);
    let mut http = Server::new(LOG_PORT,LineType::Http);
    std::thread::spawn(move ||{
        http.start();
    });
    app.start();
}