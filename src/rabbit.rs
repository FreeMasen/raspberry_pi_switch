use amiquip::{
    Connection,
    Error,
    Exchange,
    Publish,
};

pub fn send_code(code: u32, url: &str) -> Result<(), Error> {
    let mut conn = Connection::insecure_open(&url)?;
    let c =  conn.open_channel(None)?;
    let exchange = Exchange::direct(&c);
    exchange.publish(Publish::new(&code.to_be_bytes(), "lights"))?;
    Ok(())
}