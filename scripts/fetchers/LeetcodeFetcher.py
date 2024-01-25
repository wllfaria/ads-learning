import requests
from Markdown import html_to_markdown
from Problem import Problem
import FileSystem

API_URL = "https://leetcode.com/api"
GRAPHQL_URL = "https://leetcode.com/graphql"
PLATFORM = "leetcode"

def run():
    problems_to_update = FileSystem.find_problems_without_descriptions(PLATFORM)
    names_to_update = FileSystem.find_unupdated_names(PLATFORM)
    problems = fetch_problems()
    updated_problems = {}

    if len(problems_to_update) == 0:
        return

    for problem_id in problems_to_update:
        problem = problems[problem_id]
        problem.category = fetch_problem_category(problem)
        description_html = fetch_problem_description(problem)
        if description_html:
            problem.description = html_to_markdown(description_html)
        updated_problems[problem_id] = problem
        FileSystem.update_problem_description(PLATFORM, problem)


    for name in names_to_update:
        FileSystem.update_problem_name(PLATFORM, name, problems[name].slug)

    FileSystem.update_platform_summary(PLATFORM, list(updated_problems.values()))


def fetch_problems() -> dict[str, Problem]:
    problems: dict[str, Problem] = {}
    payload: list[dict] = requests.get(f"{API_URL}/problems/all/").json()[
        "stat_status_pairs"
    ]
    for problem in payload:
        problems[str(problem["stat"]["frontend_question_id"])] = Problem(
            id=problem["stat"]["frontend_question_id"],
            title=problem["stat"]["question__title"],
            slug=problem["stat"]["question__title_slug"],
            difficulty=difficulty_mapper(problem["difficulty"]["level"]),
            category="",
            url=f"https://leetcode.com/problems/{problem['stat']['question__title_slug']}",
            description="",
        )

    return problems


def fetch_problem_description(problem: Problem) -> str:
    query = {
        "query": "query questionContent($titleSlug: String!) {\n\tquestion(titleSlug: $titleSlug) {\n\t\tcontent\n\t\tmysqlSchemas\n\t\tdataSchemas\n\t}\n}\n",
        "operationName": "questionContent",
        "variables": {"titleSlug": problem.slug},
    }
    problem_content = requests.post(GRAPHQL_URL, json=query)
    return problem_content.json()["data"]["question"]["content"]


def fetch_problem_category(problem: Problem) -> str:
    query = {
        "query": "query questionTitle($titleSlug: String!) {\n\tquestion(titleSlug: $titleSlug) {\n\t\tquestionId\n\t\tquestionFrontendId\n\t\ttitle\n\t\ttitleSlug\n\t\tisPaidOnly\n\t\tdifficulty\n\t\tlikes\n\t\tdislikes\n\t\tcategoryTitle\n\t}\n}\n",
        "operationName": "questionTitle",
        "variables": {"titleSlug": problem.slug},
    }
    problem_info = requests.post(GRAPHQL_URL, json=query)
    return problem_info.json()["data"]["question"]["categoryTitle"]


def difficulty_mapper(difficulty: int) -> str:
    difficulties = {
        1: "Easy",
        2: "Medium",
        3: "Hard",
    }

    return difficulties[difficulty]
