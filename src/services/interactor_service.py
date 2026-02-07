"""Interactor service layer.

Provides reader and writer interfaces and an in-memory implementation backed by
RollbackMap for transactional staging (add/update/delete) with commit/rollback.

Caveats:
- Not suitable for multiprocessing due to shared state.
"""
from abc import ABC, abstractmethod
from typing import Optional

from interactors.interfaces import BatteryInteractor, GeneratorInteractor, ConstantActionInteractor, VariableActionInteractor
from uow.rollback_map import RollbackMap


class IInteractorServiceReader(ABC):
    """Read-only interactor service API."""
    @abstractmethod
    def get_battery_interactor(self, interactor_id: int) -> Optional[BatteryInteractor]:
        """Retrieve battery interactor details by ID."""
        ...

    @abstractmethod
    def get_generator_interactor(self, interactor_id: int) -> Optional[GeneratorInteractor]:
        """Retrieve generator interactor details by ID."""
        ...

    @abstractmethod
    def get_constant_action_interactor(self, interactor_id: int) -> Optional[ConstantActionInteractor]:
        """Retrieve constant action interactor details by ID."""
        ...

    @abstractmethod
    def get_variable_action_interactor(self, interactor_id: int) -> Optional[VariableActionInteractor]:
        """Retrieve variable action interactor details by ID."""
        ...


class IInteractorService(ABC, IInteractorServiceReader):
    """Interactor service API with mutation operations."""

    @abstractmethod
    def add_battery_interactor(self, interactor: BatteryInteractor) -> int:
        """Add a new battery interactor and return its ID."""
        ...

    @abstractmethod
    def add_generator_interactor(self, interactor: GeneratorInteractor) -> int:
        """Add a new generator interactor and return its ID."""
        ...

    @abstractmethod
    def add_constant_action_interactor(self, interactor: ConstantActionInteractor) -> int:
        """Add a new constant action interactor and return its ID."""
        ...

    @abstractmethod
    def add_variable_action_interactor(self, interactor: VariableActionInteractor) -> int:
        """Add a new variable action interactor and return its ID."""
        ...

    @abstractmethod
    def remove_interactor(self, interactor_id: int) -> None:
        """Remove an interactor by ID."""
        ...

    @abstractmethod
    def rollback(self) -> None:
        """Rollback all changes made since the last commit."""
        ...

    @abstractmethod
    def commit(self) -> None:
        """Commit all staged changes, making them permanent."""
        ...


class InteractorService(IInteractorService):
    """In-memory interactor store with transactional staging.

    Uses RollbackMap for staging changes until commit. Rollback discards staged changes.
    """
    battery_interactors: RollbackMap[BatteryInteractor]
    generator_interactors: RollbackMap[GeneratorInteractor]
    constant_action_interactors: RollbackMap[ConstantActionInteractor]
    variable_action_interactors: RollbackMap[VariableActionInteractor]

    def __init__(self):
        self.battery_interactors = RollbackMap()
        self.generator_interactors = RollbackMap()
        self.constant_action_interactors = RollbackMap()
        self.variable_action_interactors = RollbackMap()

    def get_battery_interactor(self, interactor_id: int) -> Optional[BatteryInteractor]:
        return self.battery_interactors.get(interactor_id)

    def get_generator_interactor(self, interactor_id: int) -> Optional[GeneratorInteractor]:
        return self.generator_interactors.get(interactor_id)

    def get_constant_action_interactor(self, interactor_id: int) -> Optional[ConstantActionInteractor]:
        return self.constant_action_interactors.get(interactor_id)

    def get_variable_action_interactor(self, interactor_id: int) -> Optional[VariableActionInteractor]:
        return self.variable_action_interactors.get(interactor_id)

    def add_battery_interactor(self, interactor: BatteryInteractor) -> int:
        self.battery_interactors.set(interactor.id, interactor)
        return interactor.id

    def add_generator_interactor(self, interactor: GeneratorInteractor) -> int:
        self.generator_interactors.set(interactor.id, interactor)
        return interactor.id

    def add_constant_action_interactor(self, interactor: ConstantActionInteractor) -> int:
        self.constant_action_interactors.set(interactor.id, interactor)
        return interactor.id

    def add_variable_action_interactor(self, interactor: VariableActionInteractor) -> int:
        self.variable_action_interactors.set(interactor.id, interactor)
        return interactor.id

    def remove_interactor(self, interactor_id: int) -> None:
        self.battery_interactors.delete(interactor_id)
        self.generator_interactors.delete(interactor_id)
        self.constant_action_interactors.delete(interactor_id)
        self.variable_action_interactors.delete(interactor_id)

    def rollback(self) -> None:
        self.battery_interactors.rollback()
        self.generator_interactors.rollback()
        self.constant_action_interactors.rollback()
        self.variable_action_interactors.rollback()
