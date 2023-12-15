import os


def main():
    for platform in os.listdir("problems"):
        for problem in os.listdir(f"problems/{platform}"):
            if os.path.exists(f"problems/leetcode/{problem}/README.md"):
                os.remove(f"problems/leetcode/{problem}/README.md")
            if problem.find("-") != -1 and problem.find(".") == -1:
                problem_name = problem.split("-")[0]
                os.rename(
                    f"problems/leetcode/{problem}", f"problems/leetcode/{problem_name}"
                )
        if os.path.exists(f"problems/leetcode/README.md"):
            os.remove(f"problems/leetcode/README.md")


main()
