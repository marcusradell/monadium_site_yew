use yew::Html;

use crate::articles::*;

use super::not_found;

pub fn article(id: String) -> Html {
    if id == "first_article" {
        first_article::article()
    } else if id == "hiring_junies" {
        hiring_junies::article()
    } else {
        not_found()
    }
}
