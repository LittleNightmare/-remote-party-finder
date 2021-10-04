use askama::Template;
use crate::listing_container::QueriedListing;
use std::borrow::Borrow;
use crate::sestring_ext::SeStringExt;

#[derive(Debug, Template)]
#[template(path = "listings.html")]
pub struct ListingsTemplate {
    pub containers: Vec<QueriedListing>,
}
