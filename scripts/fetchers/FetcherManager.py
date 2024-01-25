from fetchers import LeetcodeFetcher, CodeforcesFetcher

def get_fetcher(platform: str):
    fetchers = {
        "leetcode": LeetcodeFetcher,
        "codeforces": CodeforcesFetcher,
    }
    return fetchers[platform]


