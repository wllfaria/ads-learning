import logging


class Colors:
    PURPLE = "\033[95m"
    GREEN = "\033[92m"
    RED = "\033[91m"
    BOLD = "\033[1m"
    RESET = "\033[0m"


class Logger:
    def __init__(self):
        self.logger = logging.getLogger("logger")
        self.logger.setLevel(logging.INFO)
        self.logger.propagate = False
        self.logger.handlers = []

    def info(self, message: str):
        self._add_handler(self._formatter_mapping("info"))
        self.logger.info(message)
        self._reset_handlers()

    def success(self, message: str):
        self._add_handler(self._formatter_mapping("success"))
        self.logger.info(message)
        self._reset_handlers()

    def error(self, message: str):
        self._add_handler(self._formatter_mapping("error"))
        self.logger.info(message)
        self._reset_handlers()

    def debug_info(self, message: str, args: dict[str, bool]):
        if args["debug"] or args["verbose"]:
            self.info(message)

    def debug_success(self, message: str, args: dict[str, bool]):
        if args["debug"] or args["verbose"]:
            self.success(message)

    def debug_error(self, message: str, args: dict[str, bool]):
        if args["debug"] or args["verbose"]:
            self.error(message)

    def verbose_info(self, message: str, args: dict[str, bool]):
        if args["verbose"]:
            self.info(message)

    def verbose_success(self, message: str, args: dict[str, bool]):
        if args["verbose"]:
            self.success(message)

    def verbose_error(self, message: str, args: dict[str, bool]):
        if args["verbose"]:
            self.error(message)

    def _reset_handlers(self):
        self.logger.handlers = []

    def _formatter_mapping(self, kind: str) -> logging.Formatter:
        formatters = {
            "info": f"{Colors.PURPLE}{Colors.BOLD}[INFO]:{Colors.RESET} %(message)s",
            "success": f"{Colors.GREEN}{Colors.BOLD}[DONE]:{Colors.RESET} %(message)s",
            "error": f"{Colors.RED}{Colors.BOLD}[ERROR]:{Colors.RESET} %(message)s",
        }

        return logging.Formatter(formatters[kind])

    def _add_handler(self, formatter: logging.Formatter):
        handler = logging.StreamHandler()
        handler.setFormatter(formatter)
        self.logger.addHandler(handler)
