import sys
from time import sleep, time
from file_system import FileSystem
from fetcher import FetcherManager
from html_parser import HtmlParser
from logger import Logger


class Main:
    def __init__(self):
        try:
            script_start = time()
            logger = Logger()
            file_system = FileSystem()
            platforms = file_system.find_platforms()
            pluralize = lambda len, word: word if len == 1 else f"{word}s"
            args = self._parse_args()

            for platform in platforms:
                platform_start = time()
                logger.info(f"Updating {platform.capitalize()} problems")
                fetcher = FetcherManager().get_fetcher(platform)
                html_parser = HtmlParser()
                unupdated_problems = file_system.find_problems_without_descriptions(
                    platform
                )
                num_unupdated_problems = len(unupdated_problems)
                logger.info(
                    f"{num_unupdated_problems} {pluralize(num_unupdated_problems, 'problem')} pending description update"
                )
                unupdated_names = file_system.find_unupdated_names(platform)
                num_unupdated_names = len(unupdated_names)
                logger.info(
                    f"{num_unupdated_names} {pluralize(num_unupdated_names, 'problem')} pending name update"
                )
                if num_unupdated_problems == 0 and num_unupdated_names == 0:
                    logger.success(
                        f"All {platform.capitalize()} problems are up to date"
                    )
                    continue
                logger.info(f"Fetching {platform.capitalize()} problems")
                problems = fetcher.fetch_problems()
                updated_problems = []
                logger.info(f"Starting to update {platform.capitalize()} problems")

                if num_unupdated_problems == 0:
                    logger.info(f"No problems to update description, skipping")
                else:
                    for problem_id in unupdated_problems:
                        problem_start = time()
                        problem = problems[problem_id]
                        logger.debug_info(f"Updating problem {problem.title}", args)

                        problem.category = fetcher.fetch_problem_category(problem)
                        logger.verbose_info(
                            f"Fetched category for problem {problem.title}", args
                        )

                        description = fetcher.fetch_problem_description(problem)
                        logger.verbose_info(
                            f"Fetched description for problem {problem.title}", args
                        )
                        problem.description = html_parser.to_markdown(description)
                        logger.verbose_info(
                            f"Parsed html to markdown for problem {problem.title}", args
                        )

                        file_system.update_problem_description(platform, problem)
                        problem_end = time()
                        logger.debug_success(
                            f"Updated problem {problem.title} description, took {problem_end - problem_start:.2f}s",
                            args,
                        )
                        updated_problems.append(problem)
                        logger.verbose_info(f"Sleeping for 1 second", args)
                        sleep(1)

                    logger.success(f"Finished updating all descriptions")
                if num_unupdated_names == 0:
                    logger.info(f"No problems to update name, skipping")
                else:
                    for problem_id in unupdated_names:
                        problem = problems[problem_id]
                        logger.debug_info(f"Updating name of {problem.title}", args)
                        file_system.update_problem_name(
                            platform, problem_id, problem.slug
                        )
                        logger.debug_success(
                            f"Finished updating name of {problem.title}", args
                        )

                    logger.success(f"Finished updating all names")

                if num_unupdated_problems == 0:
                    logger.info(f"No problems to update summary, skipping")
                else:
                    logger.verbose_info(
                        f"Updating {platform.capitalize()} summary", args
                    )
                    file_system.update_platform_summary(platform, updated_problems)
                    logger.debug_success(
                        f"Finished updating {platform.capitalize()} summary", args
                    )
                platform_end = time()
                logger.success(
                    f"Finished updating all {platform.capitalize()} problems, took {platform_end - platform_start:.2f}s"
                )

            script_end = time()
            logger.success(
                f"All done, script finished, took {script_end - script_start:.2f}s"
            )
        except Exception as e:
            logger = Logger()
            logger.error(str(e))
            sys.exit(1)

    def _parse_args(self) -> dict[str, bool]:
        parsed_args: dict[str, bool] = {}

        if len(sys.argv) == 1:
            # prompt user if it wants to run in debug, verbose or normal mode
            print("Choose a mode:")
            print("1. Normal")
            print("2. Debug")
            print("3. Verbose")
            mode = input("Enter mode: ")
            while mode not in ["1", "2", "3"]:
                print("Invalid mode, try again")
                mode = input("Enter mode: ")
            parsed_args["debug"] = mode == "2"
            parsed_args["verbose"] = mode == "3"
        else:
            args = sys.argv[1:]
            parsed_args["verbose"] = "--verbose" in args or "-v" in args
            parsed_args["debug"] = "--debug" in args or "-d" in args

        return parsed_args


if __name__ == "__main__":
    Main()
