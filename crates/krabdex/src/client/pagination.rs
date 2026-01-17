use crate::http::Query;
use crate::types::pagination::PageRequest;

pub(crate) fn page_query(pr: PageRequest) -> Query {
    let mut q = Query::new();
    q.set("limit", pr.limit.get().to_string());
    q.set("offset", pr.offset.get().to_string());
    q
}
