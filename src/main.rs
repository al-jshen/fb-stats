use fb_stats::*;
use rayon::prelude::*;

fn main() {
    let fname =
        "/home/js/programs/fb-stats/data/messages/inbox/finessedkeeners_dbgu9woqpg/messages.json";
    // merge_files(fname, &format!("{}messages.json", fname));
    let parts = get_all_participants(fname);
    let messages = get_cleaned_messages(fname);

    let mut message_counts: Vec<usize> = Vec::with_capacity(parts.len());

    for p in &parts {
        message_counts.push(
            (&messages)
                .into_par_iter()
                .filter(|m| &m.sender_name == p)
                .count(),
        )
    }

    let total_messages = messages.len() as f64;

    let mut char_counts: Vec<usize> = Vec::with_capacity(parts.len());

    for p in &parts {
        char_counts.push(
            (&messages)
                .into_par_iter()
                .filter(|m| &m.sender_name == p)
                .map(|m| {
                    if let Some(mes) = m.content.as_ref() {
                        mes.chars().count()
                    } else {
                        0
                    }
                })
                .sum::<usize>(),
        )
    }

    let total_chars = (&char_counts).into_iter().sum::<usize>() as f64;

    println!("            Name \t\t Message %\tCharacter %");
    println!(" --------------------------------------------------------------");
    for i in 0..parts.len() {
        print!("{: >20}\t\t", parts[i]);
        print!(
            "{}%\t\t ",
            format!("{:.2}", message_counts[i] as f64 / total_messages * 100.)
        );
        println!(
            "{}%",
            format!("{:.2}", char_counts[i] as f64 / total_chars * 100.)
        );
    }

    println!("Total Messages: {:?}", message_counts);
}
