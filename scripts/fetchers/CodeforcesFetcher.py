import requests
import bs4
import FileSystem
from Problem import Problem
from Markdown import html_to_markdown

PROBLEM_URL = 'https://codeforces.com/problemset/problem'
API_URL = 'https://codeforces.com/api/problemset.problems'
PLATFORM = 'codeforces'

def run():
    problems_to_update = FileSystem.find_problems_without_descriptions(PLATFORM)
    if len(problems_to_update) == 0:
        return
    names_to_update = FileSystem.find_unupdated_names(PLATFORM)
    problems: dict[str, Problem] = {}

    for name in problems_to_update:
        problems[name] = fetch_problem(name)
        FileSystem.update_problem_description(PLATFORM, problems[name])

    if len(problems_to_update) > 0:
        FileSystem.update_platform_summary(PLATFORM, list(problems.values()))

    for name in names_to_update:
        FileSystem.update_problem_name(PLATFORM, name, problems[name].slug)


def fetch_problem(filename: str) -> Problem:
    name = filename[:-1]
    kind = filename[-1]
    problem_url = f"{PROBLEM_URL}/{name}/{kind}"
    problem = find_problem(name, kind)

    html = requests.get(problem_url).text
    soup = bs4.BeautifulSoup(html, "html.parser")

    tag = soup.find("div", "title")
    title = (tag.string if tag and isinstance(tag, bs4.Tag) and tag.string else "")
    rating = problem['rating']
    slug = title.replace(".", "").replace(" ", "-").lower()
    description = get_description(soup)

    return Problem(
        id=filename,
        title=title,
        slug=slug,
        difficulty=rating,
        category=problem['tags'][0],
        url=problem_url,
        description=html_to_markdown(description)
    )


def find_problem(name: str, kind: str) -> dict[str, str]:
    problem = {}
    all_problems = requests.get(API_URL).json()['result']['problems']
    for p in all_problems:
        if p['contestId'] == int(name) and p['index'] == kind:
            problem = p

    return problem


def get_description(soup: bs4.BeautifulSoup) -> str:
    statement = get_statement(soup)
    input = get_input_specification(soup)
    output = get_output_specification(soup)
    notes = get_notes(soup)

    return concat_contents([statement, input, output, notes])


def get_input_specification(soup: bs4.BeautifulSoup) -> str:
    div = soup.find('div', 'input-specification')
    tags = div.contents if div and isinstance(div, bs4.Tag) else []
    tags.pop(0)
    return concat_contents(tags)


def get_output_specification(soup: bs4.BeautifulSoup) -> str:
    div = soup.find('div', 'output-specification')
    tags = div.contents if div and isinstance(div, bs4.Tag) else []
    tags.pop(0)
    return concat_contents(tags)


def get_statement(soup: bs4.BeautifulSoup) -> str:
    header = soup.find('div', 'header')
    next_sibling = header.next_sibling if header else None
    contents = next_sibling.text if next_sibling else []
    return concat_contents(contents)


def get_notes(soup: bs4.BeautifulSoup) -> str:
    div = soup.find('div', 'note')
    tags = div.contents if div and isinstance(div, bs4.Tag) else []
    tags.pop(0)
    return concat_contents(tags)


def concat_contents(ls):
    return ''.join([str(i) for i in ls])
