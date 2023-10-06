import os
from directory_scanner import DirectoryScanner

class UpdateWebsitesDescriptions:
    __module__ = "update_websites_descriptions"

    def update_descriptions(self, folder, problems):
        websites = {
            "leetcode": "https://leetcode.com",
        }
        directory_scanner = DirectoryScanner()
        modules = directory_scanner.scan_directories()

        for problem in problems:
            folder_path = f"problems/{folder}"
            file_path = os.path.join(folder_path, "README.md")

            mode = "w"
            if os.path.exists(file_path):
                mode = "a"

            with open(file_path, mode) as f:
                if mode == "w":
                    f.write(f"# {folder.capitalize()} Problems\n")
                    f.write(f"Below is a list of the problems already solved in the [{folder.capitalize()}]({websites[folder]}) website:\n\n")
                    f.write("| ID | Problem Title | Link | Category | Difficulty |\n")
                    f.write("| - | - | - | - | - |\n")
                f.write(f"| [{problem['id']}]({problem['id']}/) | {problem['title']} | [{folder.capitalize()} Link]({websites[folder]}/problems/{problem['slug']}) | {problem['category']} | {problem['difficulty']} |\n")
