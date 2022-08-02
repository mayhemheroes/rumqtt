#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: (&[u8], usize)| {
    let mut bytes = bytes::BytesMut::from(data.0);
    let _ = rumqttc::mqttbytes::v4::read(&mut bytes, data.1);
});
