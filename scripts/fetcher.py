import abc

class Fetcher(abc.ABC):
    __module__ = "fetcher"


    @abc.abstractmethod
    def fetch(self, problems):
        pass
