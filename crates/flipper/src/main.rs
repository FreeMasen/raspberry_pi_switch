use amiquip::{Connection, ConsumerMessage, ConsumerOptions, QueueDeclareOptions};
mod send;

fn main() -> Result<(), String> {
    let config = lights::config::get_config()?;
    loop {
        let mut conn = Connection::insecure_open(&config.borrow().rabbit.url)
            .map_err(err)?;
        let ch = conn.open_channel(None).map_err(err)?;
        let q = ch.queue_declare(
            "lights",
            QueueDeclareOptions::default()
        ).map_err(err)?;
        let c = q.consume(ConsumerOptions::default())
            .map_err(err)?;
        for msg in c.receiver().iter() {
            match msg {
                ConsumerMessage::Delivery(m) => {
                    let mut bytes = [0;4];
                    for (n, byte) in m.body.iter().zip(bytes.iter_mut()) {
                        *byte = *n;
                    }
                    let code = u32::from_be_bytes(bytes);
                    send::send(code);
                    c.ack(m).map_err(err)?;
                },
                del => {
                    eprintln!("Error with consumer {:?}", del);
                    break;
                }
            }
        }
    }
}

fn err(e: amiquip::Error) -> String {
    format!("rabbit mq error: {}", e)
}