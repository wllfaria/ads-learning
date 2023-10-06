import markdownify


class Markdown:
    __module__ = "markdown"


    def html_to_markdown(self, html):
        markdown_text = markdownify.markdownify(html)
        return markdown_text
