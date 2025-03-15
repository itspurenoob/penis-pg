use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use rand::prelude::IndexedRandom;
use rand::prelude::IndexedMutRandom;
#[derive(Debug)]
enum WordType
{
    Noun,
    Verb,
    Adjective,
    Adverb,
}

// dictionary (not a suicidal)
const NOUNS: &[&str] = &["bass", "808", "reverb", "distortion", "phonk", "suffering", "TTS", "Rust"];
const VERBS: &[&str] = &["slaps", "destroys", "murders", "screams", "glitches", "distorts"];
const ADJECTIVES: &[&str] = &["cursed", "broken", "distorted", "epileptic", "chaotic"];
const ADVERBS: &[&str] = &["violently", "randomly", "chaotically", "loudly"];

fn get_random_word(word_type:WordType) -> &'static str
{
    let mut rng = thread_rng();
    match word_type {
        WordType::Noun => NOUNS.choose(&mut rng).unwrap(),
        WordType::Verb => VERBS.choose(&mut rng).unwrap(),
        WordType::Adjective => ADJECTIVES.choose(&mut rng).unwrap(),
        WordType::Adverb => ADVERBS.choose(&mut rng).unwrap(),
    }
}

fn generate_sentence() -> String 
{
    let structure = thread_rng().gen_range(0..4);

    match structure 
    {
        0 => format!(
            "The {} {} {}.",
            get_random_word(WordType::Adjective),
            get_random_word(WordType::Noun),
            get_random_word(WordType::Verb)
        ),
        1 => format!(
            "{} {} {} {}.",
            get_random_word(WordType::Noun),
            get_random_word(WordType::Verb),
            get_random_word(WordType::Noun),
            get_random_word(WordType::Adverb)
        ),
        2 => format!(
            "This {} {} {} {}.",
            get_random_word(WordType::Adjective),
            get_random_word(WordType::Noun),
            get_random_word(WordType::Verb),
            get_random_word(WordType::Adverb)
        ),
        _ => format!(
            "{} {} {} in a {} way.",
            get_random_word(WordType::Noun),
            get_random_word(WordType::Verb),
            get_random_word(WordType::Noun),
            get_random_word(WordType::Adjective)
        ),
    }
}

fn generate_paragraph(sentence_count: usize) -> String
{
    (0..sentence_count)
        .map(|_| generate_sentence())
        .collect::<Vec<_>>()
        .join(" ")
}

#[no_mangle]
pub extern "C" fn generate_lyrics() -> *const i8 {
    let lyrics = generate_paragraph(
        thread_rng().gen_range(5..9)
    );
    std::ffi::CString::new(lyrics).unwrap().into_raw()
}
    