from abc import ABC, abstractmethod
from enum import Enum

from electricity_price_optimizer_py import units

class ActionState(Enum):
    IDLE = "idle"
    RUNNING = "running"
    COMPLETED = "completed"


class BatteryInteractor(ABC):
    """Interface for battery device communication."""
    
    @abstractmethod
    def set_current(self, current: units.Watt) -> None:
        """Set the charge/discharge current in W (positive = charging)."""
        pass
    
    @abstractmethod
    def get_charge(self) -> units.WattHour:
        """Get the current charge level in Wh."""
        pass
    
    @abstractmethod
    def get_current(self) -> units.Watt:
        """Get the current charge/discharge rate in W."""
        pass


class GeneratorInteractor(ABC):
    """Interface for generator device communication."""
    
    @abstractmethod
    def get_current(self) -> units.Watt:
        """Get the current power generation in W."""
        pass


class ConstantActionInteractor(ABC):
    """Interface for constant action device communication."""
    
    @abstractmethod
    def start_action(self) -> None:
        """Start the action."""
        pass
    
    @abstractmethod
    def stop_action(self) -> None:
        """Stop the action (if possible)."""
        pass
    
    @abstractmethod
    def get_action_state(self) -> ActionState:
        """Get the current state of the action."""
        pass
    
    @abstractmethod
    def get_current(self) -> units.Watt:
        """Get the current power consumption in W."""
        pass


class VariableActionInteractor(ABC):
    """Interface for variable action device communication."""
    
    @abstractmethod
    def set_current(self, current: units.Watt) -> None:
        """Set the power consumption in W."""
        pass
    
    @abstractmethod
    def get_current(self) -> units.Watt:
        """Get the current power consumption in W."""
        pass
    
    @abstractmethod
    def get_total_consumed(self) -> units.WattHour:
        """Get total energy consumed so far in Wh."""
        pass
