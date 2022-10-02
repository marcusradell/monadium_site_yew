use yew::Html;

use crate::articles::first_article;

use super::not_found;

pub fn article(id: String) -> Html {
    if id != "first_article" {
        not_found()
    } else {
        first_article::article()
    }
}
