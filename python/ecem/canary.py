from collections import deque
from dataclasses import dataclass


@dataclass(frozen=True)
class CanaryStatus:
    triggered: bool
    entropy: float


class EntropyCanary:
    def __init__(self, threshold: float = 0.65, window_size: int = 5) -> None:
        self.threshold = threshold
        self.window_size = window_size
        self._history: deque[float] = deque(maxlen=window_size)

    def step(self, logprob: float) -> CanaryStatus:
        surprisal = -logprob
        normalized_score = min(max(surprisal / 4.0, 0.0), 1.0)
        self._history.append(normalized_score)

        avg_entropy = sum(self._history) / len(self._history)
        triggered = avg_entropy >= self.threshold

        return CanaryStatus(triggered=triggered, entropy=avg_entropy)

    def reset(self) -> None:
        self._history.clear()
