#[flutter_rust_bridge::frb(sync)] // Synchronous mode for simplicity of the demo
use rtcm_parser::rtcm_parser::Rtcm;

pub fn greet(name: String) -> String {
    format!("Hello, {name}!")
}

pub fn rtcm_parser_data(rtcm_data: Vec<u8>) -> String {
    let rtcm = Rtcm::parse(&rtcm_data[3..&rtcm_data.len() - 3]);

    match rtcm {
        Ok(Rtcm::Rtcm1004(message)) => {
            format!(
                r#"{{
                    "status": "success",
                    "message_type": "Rtcm1004",
                    "data": {:?}
                }}"#, message
            )
        }
        Ok(rtcm_message) => {
            format!(
                r#"{{
                    "status": "success",
                    "message_type": "Other",
                    "data": {:?}
                }}"#, rtcm_message
            )
        }
        Err(error) => {
            format!(
                r#"{{
                    "status": "error",
                    "message": "{}"
                }}"#, error
            )
        }
    }
}
// pub fn rtcm_parser_data(data: Vec<u8>) -> vec::Vec<u8> {
//     match Rtcm::parse(&data) {
//         Ok(rtcm) => format!("{}" + rtcm)
//     }
// }

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}
