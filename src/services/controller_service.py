from abc import ABC, abstractmethod
from typing import Optional
from controllers import BatteryController, GeneratorController, ConstantActionController, VariableActionController
from uow.rollback_map import RollbackMap


class IControllerServiceReader(ABC):
    @abstractmethod
    def get_battery_controller(self, controller_id: int) -> Optional[BatteryController]:
        """Retrieve battery controller details by ID."""
        ...

    @abstractmethod
    def get_generator_controller(self, controller_id: int) -> Optional[GeneratorController]:
        """Retrieve generator controller details by ID."""
        ...

    @abstractmethod
    def get_constant_action_controller(self, controller_id: int) -> Optional[ConstantActionController]:
        """Retrieve constant action controller details by ID."""
        ...

    @abstractmethod
    def get_variable_action_controller(self, controller_id: int) -> Optional[VariableActionController]:
        """Retrieve variable action controller details by ID."""
        ...


class IControllerService(ABC):
    @abstractmethod
    def add_battery_controller(self, controller: BatteryController) -> int:
        """Add a new battery controller and return its ID."""
        ...

    @abstractmethod
    def add_generator_controller(self, controller: GeneratorController) -> int:
        """Add a new generator controller and return its ID."""
        ...

    @abstractmethod
    def add_constant_action_controller(self, controller: ConstantActionController) -> int:
        """Add a new constant action controller and return its ID."""
        ...

    @abstractmethod
    def add_variable_action_controller(self, controller: VariableActionController) -> int:
        """Add a new variable action controller and return its ID."""
        ...

    @abstractmethod
    def remove_controller(self, controller_id: int) -> None:
        """Remove a controller by ID."""
        ...

    @abstractmethod
    def rollback(self) -> None:
        """Rollback all changes made since the last commit."""
        ...

    @abstractmethod
    def commit(self) -> None:
        """Commit all changes to the database."""
        ...

# IMPORTANT: Does not work in multiprocessing environments due to shared state in RollbackMap. Consider using a different approach for concurrency.


class ControllerService(IControllerService):
    battery_controllers: RollbackMap[BatteryController]
    generator_controllers: RollbackMap[GeneratorController]
    constant_action_controllers: RollbackMap[ConstantActionController]
    variable_action_controllers: RollbackMap[VariableActionController]

    def __init__(self):
        self.battery_controllers = RollbackMap()
        self.generator_controllers = RollbackMap()
        self.constant_action_controllers = RollbackMap()
        self.variable_action_controllers = RollbackMap()

    def get_battery_controller(self, controller_id: int) -> Optional[BatteryController]:
        return self.battery_controllers.get(controller_id)

    def get_generator_controller(self, controller_id: int) -> Optional[GeneratorController]:
        return self.generator_controllers.get(controller_id)

    def get_constant_action_controller(self, controller_id: int) -> Optional[ConstantActionController]:
        return self.constant_action_controllers.get(controller_id)

    def get_variable_action_controller(self, controller_id: int) -> Optional[VariableActionController]:
        return self.variable_action_controllers.get(controller_id)

    def add_battery_controller(self, controller: BatteryController) -> int:
        return self.battery_controllers.add(controller)

    def add_generator_controller(self, controller: GeneratorController) -> int:
        return self.generator_controllers.add(controller)

    def add_constant_action_controller(self, controller: ConstantActionController) -> int:
        return self.constant_action_controllers.add(controller)

    def add_variable_action_controller(self, controller: VariableActionController) -> int:
        return self.variable_action_controllers.add(controller)

    def remove_controller(self, controller_id: int) -> None:
        self.battery_controllers.remove(controller_id)
        self.generator_controllers.remove(controller_id)
        self.constant_action_controllers.remove(controller_id)
        self.variable_action_controllers.remove(controller_id)

    def rollback(self) -> None:
        self.battery_controllers.rollback()
        self.generator_controllers.rollback()
        self.constant_action_controllers.rollback()
        self.variable_action_controllers.rollback()

    def commit(self) -> None:
        self.battery_controllers.commit()
        self.generator_controllers.commit()
        self.constant_action_controllers.commit()
        self.variable_action_controllers.commit()
