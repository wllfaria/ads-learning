class Problem:
    def __init__(
        self,
        title: str,
        id: str,
        slug: str,
        category: str,
        difficulty: str,
        url: str,
        description: str,
    ) -> None:
        self.title = title
        self.id = id
        self.slug = slug
        self.category = category
        self.difficulty = difficulty
        self.url = url
        self.description = description

