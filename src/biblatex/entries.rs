use biblatex::{Field, NameList, Range};

/// A bibliographic entry
#[derive(PartialEq, Debug)]
pub enum Entry {
    /// Article in a journal or other periodical forming a self-contained unit
    Article(Article),
    /// Article in a conference proceedings
    InProceedings(InProceedings),
}

impl Entry {
    pub fn author(&self) -> &NameList {
        match *self {
            Entry::InProceedings(InProceedings { ref author, .. }) |
            Entry::Article(Article { ref author, ..}) => author,
        }
    }

    pub fn title(&self) -> &str {
        match *self {
            Entry::InProceedings(InProceedings { ref title, .. }) |
            Entry::Article(Article { ref title, ..}) => title,
        }
    }

    pub fn year(&self) -> i32 {
        match *self {
            Entry::InProceedings(InProceedings { year, .. }) |
            Entry::Article(Article { year, ..}) => year,
        }
    }

    /// Attempts to instantiate a new `InProceedings`
    pub fn new_in_proceedings(fields: Vec<Field>) -> Option<Entry> {
        let mut inproc = InProceedings::default();

        // required fields
        let mut has_year = false;
        let mut has_title = false;
        let mut has_author = false;
        let mut has_book_title = false;

        for field in fields {
            match field {
                Field::Year(y) => {
                    inproc.year = y;
                    has_year = true;
                }
                Field::Title(s) => {
                    inproc.title = s;
                    has_title = true;
                }
                Field::BookTitle(s) => {
                    inproc.book_title = s;
                    has_book_title = true;
                }
                Field::Author(a) => {
                    inproc.author = a;
                    has_author = true;
                }
                Field::Pages(ps) => inproc.pages = Some(ps),
                Field::Editor(es) => inproc.editor = Some(es),
                Field::Volume(v) => inproc.volume = Some(v),
                Field::Series(s) => inproc.series = Some(s),
                Field::Url(u) => inproc.url = Some(u),
                _ => (),
            }
        }

        if has_year && has_title && has_author && has_book_title {
            Some(Entry::InProceedings(inproc))
        } else {
            None
        }
    }

    /// Attempts to instantiate a new `Article`
    pub fn new_article(fields: Vec<Field>) -> Option<Entry> {
        let mut article = Article::default();

        // required fields
        let mut has_year = false;
        let mut has_title = false;
        let mut has_author = false;
        let mut has_journal_title = false;

        for field in fields {
            match field {
                Field::Year(y) => {
                    article.year = y;
                    has_year = true;
                }
                Field::Title(s) => {
                    article.title = s;
                    has_title = true;
                }
                Field::JournalTitle(s) => {
                    article.journal_title = s;
                    has_journal_title = true
                }
                Field::Author(a) => {
                    article.author = a;
                    has_author = true;
                }
                Field::Pages(ps) => article.pages = Some(ps),
                Field::Editor(es) => article.editor = Some(es),
                Field::Volume(v) => article.volume = Some(v),
                Field::Series(s) => article.series = Some(s),
                Field::Issue(i) => article.issue = Some(i),
                Field::Url(u) => article.url = Some(u),
                _ => (),
            }
        }

        if has_year && has_title && has_author && has_journal_title {
            Some(Entry::Article(article))
        } else {
            None
        }
    }
}

#[derive(PartialEq, Debug, Default)]
pub struct InProceedings {
    pub author: NameList,
    pub title: String,
    pub year: i32,
    pub book_title: String,
    pub editor: Option<NameList>,
    pub volume: Option<String>,
    pub series: Option<String>,
    pub pages: Option<Vec<Range>>,
    pub url: Option<String>,
}

#[derive(PartialEq, Debug, Default)]
pub struct Article {
    pub author: NameList,
    pub title: String,
    pub year: i32,
    pub journal_title: String,
    pub editor: Option<NameList>,
    pub volume: Option<String>,
    pub series: Option<String>,
    pub issue: Option<String>,
    pub pages: Option<Vec<Range>>,
    pub url: Option<String>,
}