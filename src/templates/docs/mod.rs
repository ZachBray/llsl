use output::Template;

pub fn readme() -> Template {
    Template {
        name: "readme",
        content: include_str!("readme.hbs"),
        render_targets: Box::new(|protocol, renderer| renderer.render("README.md", protocol)),
    }
}
