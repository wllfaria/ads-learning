import os
from Problem import Problem

def find_platforms() -> list[str]:
    platforms: list[str] = []
    for folder in os.listdir("problems"):
        platforms.append(folder)
    return platforms


def find_problems_without_descriptions(platform: str) -> list[str]:
    problems_without_descriptions: list[str] = []
    for problem in os.listdir(f"problems/{platform}"):
        if problem.find(".") == -1 and not os.path.exists(
            f"problems/{platform}/{problem}/README.md"
        ):
            if problem.find("-") != -1:
                problem = problem.split("-")[0]
            problems_without_descriptions.append(problem)
    return problems_without_descriptions


def find_unupdated_names(platform: str) -> list[str]:
    unupdated_names: list[str] = []
    for problem in os.listdir(f"problems/{platform}"):
        if problem.find("-") == -1 and problem.find(".") == -1:
            unupdated_names.append(problem)
    return unupdated_names


def update_problem_description(platform: str, problem: Problem) -> None:
    file_path = ""
    if os.path.exists(f"problems/{platform}/{problem.id}"):
        file_path = f"problems/{platform}/{problem.id}/README.md"
    else:
        file_path = f"problems/{platform}/{problem.id}-{problem.slug}/README.md"
    with open(file_path, "w") as f:
        f.write(f"# {problem.title}\n")
        f.write(f"* ID: {problem.id}\n")
        f.write(f"* [View on {platform.capitalize()}]({problem.url})\n")
        f.write(f"* Category: {problem.category}\n")
        f.write(f"* Difficulty: {problem.difficulty}\n\n")
        f.write(problem.description)


def update_problem_name(platform: str, old_name: str, new_name: str) -> None:
    old_path = f"problems/{platform}/{old_name}"
    new_path = f"problems/{platform}/{old_name}-{new_name}"
    os.rename(old_path, new_path)


def update_platform_summary(platform: str, problems: list[Problem]):
    for problem in problems:
        edit_mode = "w"
        if os.path.exists(f"problems/{platform}/README.md"):
            edit_mode = "a"

        with open(f"problems/{platform}/README.md", edit_mode) as f:
            if edit_mode == "w":
                f.write(f"# {platform.capitalize()} Problems\n")
                f.write(
                    f"Below is a list of the problems already solved in the [{platform.capitalize()}]({problem.url}) website:\n\n"
                )
                f.write("| ID | Problem Title | Link | Category | Difficulty |\n")
                f.write("| - | - | - | - | - |\n")
            f.write(
                f"| [{problem.id}]({problem.id}-{problem.slug}/) | {problem.title} | [{platform.capitalize()} Link]({problem.url}) | {problem.category} | {problem.difficulty} |\n"
            )
