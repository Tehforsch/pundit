pub static LINK_FORMAT: &str = "[[file:{relative_path}][{title}]]";
pub static TITLE_STRING: &str = "#+TITLE: ";
pub static NOTE_EXTENSION: &str = "org";
pub static NOTE_FILENAME_STR_FORMAT: &str = "{dateString}-{titleString}.org";
pub static NOTE_DATE_FORMAT_STR: &str = "%Y%m%d%H%M%S";
pub static ID_MULTIPLIER: i64 = 100;
pub static ANKI_BLOCK_NOTE_TEMPLATE: &str = "
{id}:
{fields}
";
pub static ANKI_FULL_NOTE_TEMPLATE: &str = "#+begin_src yaml
deck: {deck}
model: {model}
{id}:
{fields}
#+end_src";
pub static ANKI_NOTE_FIELD_TEMPLATE: &str = "    {fieldName}: ";
pub static JOURNAL_TITLE_FORMAT: &str = "{} {}";
pub static JOURNAL_DATE_FORMAT_STR: &str = "%Y %m %d";
pub static JOURNAL_IN_SUBFOLDERS: bool = true;

pub static PAPER_NOTE_TITLE: &str = "paper";
pub static PAPER_FOLDER_NAME: &str = "papers";
