use fb_stats::*;

fn main() {
    let messages = load_messages(
        "/home/js/programs/fb-stats/data/messages/inbox/finessedkeeners_dbgu9woqpg/message_1.json"
            .to_owned(),
    );
    for m in messages {
        if m.content.is_none() {
            println!("{:?}", m);
        }
    }
}
