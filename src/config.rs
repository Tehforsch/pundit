pub static LINK_FORMAT: &str = "[[file:{relative_path}][{title}]]";
pub static TITLE_STRING: &str = "#+TITLE: ";
pub static NOTE_EXTENSION: &str = "org";
pub static NOTE_FILENAME_STR_FORMAT: &str = "{dateString}-{titleString}.org";
pub static NOTE_DATE_FORMAT_STR: &str = "%Y%m%d%H%M%S";
pub static ID_MULTIPLIER: i64 = 100;
pub static ANKI_NOTE_HEADER_TEMPLATE: &str = "#anki {id} {model} {deck}";
// pub static ANKI_NOTE_HEADER_TEMPLATE: &str = "#+BEGIN_SRC yaml\n
// {id}:
//     deck: {deck}
//     model: {model}
//     Spanish: la palabra en Español
//     English: the word in English
// #+END_SRC
pub static ANKI_NOTE_FIELD_TEMPLATE: &str = "#{fieldName}";
pub static JOURNAL_TITLE_FORMAT: &str = "{} {}";
pub static JOURNAL_DATE_FORMAT_STR: &str = "%Y %m %d";
pub static JOURNAL_IN_SUBFOLDERS: bool = true;
