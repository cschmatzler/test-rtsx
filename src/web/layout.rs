use maud::{html, Markup, DOCTYPE};

pub fn render(title: &'static str, inner: Markup) -> Markup {
	html! {
		(DOCTYPE)
		html lang="en" {
			head {
				meta charset="utf-8";
				meta name="viewport" content="width=device-width, initial-scale=1.0";
				title { (title) }
				script src="https://unpkg.com/htmx.org@1.8.4" {}
				link rel="stylesheet" href="/assets/styles.css";
			}
			body {
				(inner)
			}
		}
	}
}
