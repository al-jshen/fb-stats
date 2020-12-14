use fb_stats::*;
use rayon::prelude::*;

fn main() {
    let fname =
        "/home/js/programs/fb-stats/data/messages/inbox/finessedkeeners_dbgu9woqpg/messages.json";
    let parts = get_all_participants(fname);
    let messages = get_cleaned_messages(fname);

    let total_messages = messages.len() as f64;

    for p in parts {
        println!(
            "{} {}",
            p,
            (&messages)
                .into_par_iter()
                .filter(|m| m.sender_name == p)
                .count() as f64
                / total_messages
        );
    }
}
