use colored::*;
use rand::seq::SliceRandom;

fn main() {
    let quotes = vec![
        ("The only way to do great work is to love what you do.", "Steve Jobs"),
        ("Believe you can and you're halfway there.", "Theodore Roosevelt"),
        ("The future belongs to those who believe in the beauty of their dreams.", "Eleanor Roosevelt"),
        ("It does not matter how slowly you go as long as you do not stop.", "Confucius"),
        ("Everything you've ever wanted is on the other side of fear.", "George Addair"),
        ("Success is not final, failure is not fatal: it is the courage to continue that counts.", "Winston Churchill"),
        ("Believe in yourself. You are braver than you think, more talented than you know, and capable of more than you imagine.", "Roy T. Bennett"),
        ("I learned that courage was not the absence of fear, but the triumph over it.", "Nelson Mandela"),
        ("The only impossible journey is the one you never begin.", "Tony Robbins"),
        ("Your limitation—it's only your imagination.", "Unknown"),
        ("Push yourself, because no one else is going to do it for you.", "Unknown"),
        ("Great things never come from comfort zones.", "Unknown"),
        ("Dream it. Wish it. Do it.", "Unknown"),
        ("Success doesn't just find you. You have to go out and get it.", "Unknown"),
        ("The harder you work for something, the greater you'll feel when you achieve it.", "Unknown"),
        ("Dream bigger. Do bigger.", "Unknown"),
        ("Don't stop when you're tired. Stop when you're done.", "Unknown"),
        ("Wake up with determination. Go to bed with satisfaction.", "Unknown"),
        ("Do something today that your future self will thank you for.", "Unknown"),
        ("Little things make big days.", "Unknown"),
        ("It's going to be hard, but hard does not mean impossible.", "Unknown"),
        ("Don't wait for opportunity. Create it.", "Unknown"),
        ("The universe is always speaking to us... sending us little messages, causing coincidences and serendipities.", "Deepak Chopra"),
        ("What you seek is seeking you.", "Rumi"),
        ("Everything is energy and that's all there is to it.", "Albert Einstein"),
        ("Everything is working out for me.", "Manifestation"),
        ("My person is coming to me now.", "Universe"),
        ("I am aligned with my highest self.", "Affirmation"),
        ("What's meant for me will never miss me.", "Trust"),
    ];

    let mut rng = rand::thread_rng();
    let (quote, author) = quotes.choose(&mut rng).unwrap();

    println!("\n{}", "✨ Quote of the Day ✨".bright_cyan().bold());
    println!("\n  {}", format!("\"{}\"", quote).bright_white());
    println!("\n  {} {}\n", "—".bright_black(), author.bright_yellow());
}
