use lucian::config::Config;
use lucian::gate::hub::line_header::LineType;
use lucian::log::Log;
use lucian::server::Server;

pub fn start() {
    let (app_addr,http_addr,worker,proxy_server_addr,write_log) = Config::get_all();
    if write_log {
        Log::turn_on();
    }
    
    let mut app = Server::new(app_addr,LineType::Fox);
    app.init(worker,proxy_server_addr);
    let mut http = Server::new(http_addr,LineType::Http);
    std::thread::spawn(move ||{
        http.start();
    });
    app.start();
}