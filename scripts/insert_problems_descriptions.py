import os
from directory_scanner import DirectoryScanner
from leetcode_fetcher import LeetcodeFetcher
from update_websites_descriptions import UpdateWebsitesDescriptions


def main():
    websites = {
        "leetcode": "https://leetcode.com/problems",
    }
    fetchers = {
        "leetcode": LeetcodeFetcher(),
    }
    scanner = DirectoryScanner()
    modules = scanner.get_missing_descriptions()

    for key, value in modules.items():
        payload = fetchers[key].fetch(value)
        modules[key] = payload

    for key in modules:
        if len(modules[key]) <= 0:
            continue

        website_folder_path = f"problems/{key}"
        website_description_updater = UpdateWebsitesDescriptions()
        website_description_updater.update_descriptions(key, modules[key])

        for problem in modules[key]:
            folder_path = f"{website_folder_path}/{problem['id']}"
            file_path = os.path.join(folder_path, "README.md")

            with open(file_path, "w") as f:
                f.write(f"# {problem['title']}\n")
                f.write(f"* ID: {problem['id']}\n")
                f.write(f"* [Problem on Website]({websites[key]}/{problem['slug']})\n")
                f.write(f"* Category: {problem['category']}\n")
                f.write(f"* Difficulty: {problem['difficulty']}\n\n")
                f.write(problem['content'])







if __name__ == "__main__":
    main()
