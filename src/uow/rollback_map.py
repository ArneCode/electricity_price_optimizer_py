from typing import TypeVar, Generic, Dict, Set, Optional

# T represents the generic type of the object you're storing
T = TypeVar('T')


class RollbackMap(Generic[T]):
    def __init__(self):
        # The "source of truth"
        self._committed_state: Dict[int, T] = {}

        # Buffers for the current transaction
        self._staged_changes: Dict[int, T] = {}
        self._staged_deletions: Set[int] = set()

    def get(self, key: int) -> Optional[T]:
        """O(1) lookup: Checks staged changes first, then committed data."""
        if key in self._staged_deletions:
            return None
        if key in self._staged_changes:
            return self._staged_changes[key]
        return self._committed_state.get(key)

    def set(self, key: int, value: T) -> None:
        """O(1) write: Records the change in the staged buffer."""
        if key in self._staged_deletions:
            self._staged_deletions.remove(key)
        self._staged_changes[key] = value

    def delete(self, key: int) -> None:
        """O(1) deletion: Marks the key for removal upon commit."""
        if key in self._staged_changes:
            del self._staged_changes[key]

        # Only need to track deletion if it actually exists in the committed state
        if key in self._committed_state:
            self._staged_deletions.add(key)

    def commit(self) -> None:
        """O(K) where K is the number of changes since last commit."""
        # Apply additions/updates
        self._committed_state.update(self._staged_changes)

        # Apply deletions
        for key in self._staged_deletions:
            self._committed_state.pop(key, None)

        self.rollback()  # Clear the buffers

    def rollback(self) -> None:
        """O(1) - Simply wipes the staged buffers."""
        self._staged_changes.clear()
        self._staged_deletions.clear()

    def __repr__(self) -> str:
        # Merges view for debugging/printing
        current = self._committed_state.copy()
        current.update(self._staged_changes)
        for k in self._staged_deletions:
            current.pop(k, None)
        return f"TransactionalMap({current})"
