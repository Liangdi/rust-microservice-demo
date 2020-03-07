use crate::NACOS_SERVER;
use crate::PROVIDER_HOST;
use crate::PROVIDER_NAME;
use crate::PROVIDER_PORT;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use std::time::Duration;
use async_std::task;

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'{').add(b'}').add(b':').add(b',');
///
/// https://nacos.io/
/// http://127.0.0.1:8848/nacos/v1/ns/instance?serviceName=rust-microservice&ip=127.0.0.1&port=8080
pub fn register_service() {
    println!("register service: {:?}", NACOS_SERVER);

    task::spawn(
        async {
            let client = reqwest::blocking::Client::new();
            let body = client.post(
                format!("{}/v1/ns/instance?serviceName={}&ip={}&port={}",
                        NACOS_SERVER,
                        PROVIDER_NAME,
                        PROVIDER_HOST,
                        PROVIDER_PORT).as_str()
            ).send().unwrap().text();
            println!("{:?}", body);
        }
    );
}
fn ping() {
    //
    // nacos 文档中没有说明 metadata 必选, 测试发现，如果没有 metadata 信息， java 端会有错误
    //
    let beat = format!("{{\"serviceName\":\"{}\",\"ip\":\"{}\",\"port\":\"{}\",\"weight\":1,\"metadata\":{{}}}}", PROVIDER_NAME, PROVIDER_HOST, PROVIDER_PORT);
    let  encode = utf8_percent_encode(&beat, FRAGMENT).to_string();
    task::spawn(
        async move {

            let client = reqwest::blocking::Client::new();
            let _body = client.put(
                format!("{}/v1/ns/instance/beat?serviceName={}&beat={}",
                        NACOS_SERVER,
                        PROVIDER_NAME,
                        encode
                ).as_str()
            ).send().unwrap().text();
            println!("ping result:{:?}", _body);
        }
    );
}


pub fn ping_schedule() {
    println!("ping schedule");
    loop {
        ping();
        std::thread::sleep(Duration::from_secs(1));
    }
}