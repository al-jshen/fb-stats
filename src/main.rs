use fb_stats::*;

fn main() {
    merge_files(
        "/home/js/programs/fb-stats/data/messages/inbox/finessedkeeners_dbgu9woqpg/",
        r"message_\d+.json$",
        "/home/js/programs/fb-stats/data/messages/inbox/finessedkeeners_dbgu9woqpg/messages.json",
    );
}
