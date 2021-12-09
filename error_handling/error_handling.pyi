from typing import Tuple

def rust_log(x: float) -> float:
    ...

def rust_exp(x: float) -> float:
    ...

def rust_div(x: float, y: float) -> float:
    ...


def rust_log_with_exp(x: float) -> Tuple[float, float]:
    ...

def rust_log_with_exp2(x: float) -> Tuple[float, float]:
    ...

def rust_handle_error(x: float) -> int:
    ...


class MathError(Exception):
    ...

class LogSmallError(MathError):
    ...

class ExpLargeError(MathError):
    ...