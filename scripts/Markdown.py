import markdownify


def html_to_markdown(html: str) -> str:
    markdown_text: str = markdownify.markdownify(html)
    return markdown_text
