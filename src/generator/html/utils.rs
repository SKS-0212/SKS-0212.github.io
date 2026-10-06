use pulldown_cmark::{
    html,
    Event,
    HeadingLevel,
    Parser,
    Tag,
    TagEnd,
};


pub(super) fn render_markdown(markdown: &str) -> String {
    let mut html_output = String::new();
    let parser = Parser::new(markdown);

    let transformed = parser.map(|event| {
        match event {
            Event::Start(Tag::Heading {
                             level, id, classes, attrs })
            => Event::Start(Tag::Heading {
                level: shift_heading_level(level), id, classes, attrs, }),
            Event::End(TagEnd::Heading(level)) => {
                Event::End(TagEnd::Heading(shift_heading_level(level)))
            }
            event => event,
        }
    });

    html::push_html(&mut html_output, transformed);
    html_output
}


pub(super) fn escape_html(input: &str) -> String {
    input
        .replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
        .replace("'", "&#39;")

}


fn shift_heading_level(level: HeadingLevel)-> HeadingLevel{
    match level {
        HeadingLevel::H1 => HeadingLevel::H4,
        HeadingLevel::H2 => HeadingLevel::H5,
        HeadingLevel::H3 => HeadingLevel::H6,

        HeadingLevel::H4 | HeadingLevel::H5 | HeadingLevel::H6 => HeadingLevel::H6
    }
}
