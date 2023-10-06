import os

class DirectoryScanner:
    __module__ = "directory_scanner"

    problems_folder = "problems"


    def scan_directories(self):
        modules = {}
        for folder in os.listdir(self.problems_folder):
            problems = os.listdir(f"{self.problems_folder}/{folder}")
            modules[folder] = {}
            for problem in problems:
                if os.path.isfile(f"{self.problems_folder}/{folder}/{problem}"):
                    continue
                files = os.listdir(f"{self.problems_folder}/{folder}/{problem}")
                if "README.md" in files:
                    modules[folder][problem] = True
                else:
                    modules[folder][problem] = False
                if "solution.exe" in files:
                    self._delete_solution_binary(folder, problem)
        return modules


    def _delete_solution_binary(self, folder, problem):
        os.remove(f"{self.problems_folder}/{folder}/{problem}/solution.exe")


    def get_missing_descriptions(self):
        modules = self.scan_directories()
        return modules


