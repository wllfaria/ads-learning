import markdownify


class HtmlParser:
    def to_markdown(self, html: str) -> str:
        markdown_text: str = markdownify.markdownify(html)
        return markdown_text
