import FileSystem
from fetchers import FetcherManager

def run():
    platforms = FileSystem.find_platforms()

    for platform in platforms:
        fetcher = FetcherManager.get_fetcher(platform)
        fetcher.run()

if __name__ == '__main__':
    run()
