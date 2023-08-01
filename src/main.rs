mod eml;

use crate::eml::Attachment;
use eml::Eml;
use mail_parser::*;

fn debug_eml(message: &Eml) {
    println!("\n");
    println!("Embedded message attachment: ");
    println!("Message body: {:?}", message.body);

    for attachment in &message.attachments {
        match attachment {
            Attachment::Eml(eml) => {
                debug_eml(&eml);
                println!("End of embedded message\n");
            }
            Attachment::Other {
                name,
                content_type,
                contents: _,
            } => {
                println!("Name: {:?}, content_type: {:?}", name, content_type);
            }
        }
    }
}

fn main() {
    let filename = "message5.eml";
    let file_content = std::fs::read(format!("eml/{filename}")).expect("Could not read file");
    let message: Eml = Message::parse(&file_content)
        .expect("Could not parse the file")
        .into();

    debug_eml(&message);
}
