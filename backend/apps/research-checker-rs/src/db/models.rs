use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable, Debug)]
#[diesel(table_name = crate::schema::retraction)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Retraction {
    pub id: i32,
    pub title: Option<String>,
    pub journal: Option<String>,
    pub link: Option<String>,
    pub doi: Option<String>,
    pub pubmed_id: Option<String>,
    pub affiliations: Vec<Option<String>>,
    pub countries: Vec<Option<String>>,
    pub authors: Vec<Option<String>>,
    pub issues: Vec<Option<String>>,
}
