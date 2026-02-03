from abc import ABC, abstractmethod


class IInteractorService(ABC):
    @abstractmethod
    def get_interactor(self, interactor_id: int) -> object:
        """Retrieve interactor details by ID."""
        ...

    @abstractmethod
    def add_interactor(self, interactor: object) -> int:
        """Add a new interactor and return its ID."""
        ...

    @abstractmethod
    def remove_interactor(self, interactor_id: int) -> None:
        """Remove an interactor by ID."""
        ...


class InteractorService(IInteractorService):
    def get_interactor(self, interactor_id: int) -> object:
        # Placeholder implementation
        pass

    def add_interactor(self, interactor: object) -> int:
        # Placeholder implementation
        pass

    def remove_interactor(self, interactor_id: int) -> None:
        # Placeholder implementation
        pass
