from abc import ABC, abstractmethod


class IControllerService(ABC):
    @abstractmethod
    def get_controller(self, controller_id: int) -> object:
        """Retrieve controller details by ID."""
        ...

    @abstractmethod
    def add_controller(self, controller: object) -> int:
        """Add a new controller and return its ID."""
        ...

    @abstractmethod
    def remove_controller(self, controller_id: int) -> None:
        """Remove a controller by ID."""
        ...
