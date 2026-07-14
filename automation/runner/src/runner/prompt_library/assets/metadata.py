from __future__ import annotations


def strip_asset_frontmatter(markdown: str) -> str:
    if not markdown.startswith("---\n"):
        return markdown
    separator = markdown.find("\n---\n", 4)
    if separator < 0:
        return markdown
    return markdown[separator + 5 :]
