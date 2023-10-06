import time
import requests
from fetcher import Fetcher
from markdown import Markdown


class LeetcodeFetcher(Fetcher):
    _rest_api_url = "https://leetcode.com/api"
    _graphql_api_url = "https://leetcode.com/graphql"
    _all_problems_url = f"{_rest_api_url}/problems/all/"


    def __init__(self):
        self._markdown = Markdown()


    def fetch(self, problems):
        if len(problems) <= 0:
            return []
        all_problems = requests.get(self._all_problems_url).json()["stat_status_pairs"]
        shaped_problems = []
        for problem, has_description in problems.items():
            if has_description:
                continue
            for problem_data in all_problems:
                if problem_data["stat"]["question_id"] == int(problem):
                    problem_content = problem_data["stat"]
                    problem_content_data = self._fetch_problem_content(problem_data["stat"]["question__title_slug"])
                    problem_content["content"] = self._markdown.html_to_markdown(problem_content_data)
                    time.sleep(1)
                    problem_content["info"] = self._fetch_problem_info(problem_data["stat"]["question__title_slug"])
                    time.sleep(1)
                    shaped_problems.append(problem_content)
                    break
        payload = self._create_payload(shaped_problems)
        return payload


    def _fetch_problem_content(self, problem_slug):
        query = {
            "query": "query questionContent($titleSlug: String!) {\n\tquestion(titleSlug: $titleSlug) {\n\t\tcontent\n\t\tmysqlSchemas\n\t\tdataSchemas\n\t}\n}\n",
            "operationName":"questionContent",
            "variables": {"titleSlug": problem_slug}
        }
        problem_content = requests.post(self._graphql_api_url, json=query)
        return problem_content.json()["data"]["question"]["content"]


    def _fetch_problem_info(self, problem_slug):
        query = {
            "query": "query questionTitle($titleSlug: String!) {\n\tquestion(titleSlug: $titleSlug) {\n\t\tquestionId\n\t\tquestionFrontendId\n\t\ttitle\n\t\ttitleSlug\n\t\tisPaidOnly\n\t\tdifficulty\n\t\tlikes\n\t\tdislikes\n\t\tcategoryTitle\n\t}\n}\n",
            "operationName":"questionTitle",
            "variables":{"titleSlug":"valid-palindrome"}
        }
        problem_info = requests.post(self._graphql_api_url, json=query)
        return problem_info.json()["data"]["question"]


    def _create_payload(self, problems_data):
        payload = []
        for problem in problems_data:
            payload.append({
                "id": problem["question_id"],
                "title": problem["question__title"],
                "slug": problem["question__title_slug"],
                "difficulty": problem["info"]["difficulty"],
                "category": problem["info"]["categoryTitle"],
                "content": problem["content"],
            })
        return payload
