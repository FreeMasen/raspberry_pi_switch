

#[cfg(target_arch = "arm")]
pub fn send(code: u32) {
    codesender::send(code);
}

#[cfg(not(target_arch = "arm"))]
pub fn send(code: u32) {
    println!("send: {}", code);
}