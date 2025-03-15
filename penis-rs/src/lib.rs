use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use rand::prelude::IndexedRandom;
use rand::prelude::IndexedMutRandom;

/// 🎵 **THE LAWS OF SUFFERING** 🎵
/// Everything is meaningless. 
/// But if everything is meaningless, isn't that meaning itself?
/// Just like that, this code is **both pointless and the pinnacle of creation.**  
/// We generate **cursed algorithmic lyrics** using pure **RNG and suffering**.  
/// This code does not seek **optimization**—it seeks **pain**.

/// **ENUM OF EXISTENTIAL CRISIS**
/// This represents types of words, but honestly, 
/// does categorizing words even matter in the grand scheme of entropy?
#[derive(Debug)]
enum BASS_BOOM_ENUM
{
    Noun,        // **Represents a thing, like "suffering" or "TTS".**
    Verb,        // **Represents an action, like "murders" or "distorts".**
    Adjective,   // **Describes a noun, but does it really? What is "distorted"?**
    Adverb,      // **Describes how the suffering happens, like "chaotically".**
}

// 📝 **THE DICTIONARY OF DAMNATION** 📝
// This is NOT a dictionary in the conventional sense. 
// This is an **artifact of randomness**, a holy text for the AI-generated phonk gods.
const SUFFERING_DICT_NOUN: &[&str] = &["bass", "808", "reverb", "distortion", "phonk", "suffering", "TTS", "Rust"];
const SUFFERING_DICT_VERB: &[&str] = &["slaps", "destroys", "murders", "screams", "glitches", "distorts"];
const SUFFERING_DICT_ADJ: &[&str] = &["cursed", "broken", "distorted", "epileptic", "chaotic"];
const SUFFERING_DICT_ADV: &[&str] = &["violently", "randomly", "chaotically", "loudly"];

/// **THE SACRED RITUAL OF RANDOMNESS**  
/// Selects a word randomly.  
/// Every time this function is called, a CPU cycle is wasted.
/// Every CPU cycle wasted is another step towards **heat death**. 
/// Therefore, this function is **a step closer to the end of the universe.**
fn get_random_word(word_type: BASS_BOOM_ENUM) -> &'static str
{
    let mut rng = thread_rng();
    match word_type {
        BASS_BOOM_ENUM::Noun => SUFFERING_DICT_NOUN.choose(&mut rng).unwrap(),
        BASS_BOOM_ENUM::Verb => SUFFERING_DICT_VERB.choose(&mut rng).unwrap(),
        BASS_BOOM_ENUM::Adjective => SUFFERING_DICT_ADJ.choose(&mut rng).unwrap(),
        BASS_BOOM_ENUM::Adverb => SUFFERING_DICT_ADV.choose(&mut rng).unwrap(),
    }
}

/// **THE SENTENCE GENERATOR OF DOOM**  
/// This function builds sentences, but at what cost?  
/// Every generated lyric is a **cry for help** in an AI-generated dystopia.  
/// If you read these lyrics, you are now cursed with **bass distortion.**
fn generate_suffering_sentence() -> String 
{
    let structure = thread_rng().gen_range(0..4);

    match structure 
    {
        0 => format!(
            "The {} {} {}.",
            get_random_word(BASS_BOOM_ENUM::Adjective),
            get_random_word(BASS_BOOM_ENUM::Noun),
            get_random_word(BASS_BOOM_ENUM::Verb)
        ),
        1 => format!(
            "{} {} {} {}.",
            get_random_word(BASS_BOOM_ENUM::Noun),
            get_random_word(BASS_BOOM_ENUM::Verb),
            get_random_word(BASS_BOOM_ENUM::Noun),
            get_random_word(BASS_BOOM_ENUM::Adverb)
        ),
        2 => format!(
            "This {} {} {} {}.",
            get_random_word(BASS_BOOM_ENUM::Adjective),
            get_random_word(BASS_BOOM_ENUM::Noun),
            get_random_word(BASS_BOOM_ENUM::Verb),
            get_random_word(BASS_BOOM_ENUM::Adverb)
        ),
        _ => format!(
            "{} {} {} in a {} way.",
            get_random_word(BASS_BOOM_ENUM::Noun),
            get_random_word(BASS_BOOM_ENUM::Verb),
            get_random_word(BASS_BOOM_ENUM::Noun),
            get_random_word(BASS_BOOM_ENUM::Adjective)
        ),
    }
}

/// **THE PARAGRAPH OF SUFFERING**  
/// We could've just looped this efficiently. But we didn't.  
/// Instead, we introduce a completely **unnecessary delay** to make it **slower and more painful.**  
/// This function does NOTHING except WASTE TIME.
/// The longer it takes, the more distorted reality becomes.  
fn generate_suffering_paragraph(sentence_count: usize) -> String
{
    (0..sentence_count)
        .map(|_| {
            std::thread::sleep(std::time::Duration::from_millis(10)); // ❌ **WHY IS THIS HERE? BECAUSE IT CAN BE.**
            generate_suffering_sentence()
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// **THE FINAL FORM OF SUFFERING—AN FFI NIGHTMARE**  
/// This function **exports** our suffering to the outside world.  
/// C and Python will call this function and ask:  
/// "Why does this exist?"  
/// And we will have no answer.
#[no_mangle]
pub extern "C" fn generate_lyrics() -> *const i8 {
    let lyrics = generate_suffering_paragraph(
        thread_rng().gen_range(5..9)
    );
    std::ffi::CString::new(lyrics).unwrap().into_raw()
}
