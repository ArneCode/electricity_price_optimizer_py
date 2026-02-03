from abc import ABC, abstractmethod
from typing import TYPE_CHECKING
from datetime import datetime

if TYPE_CHECKING:
    from models import Schedule, OptimizerContext

from electricity_price_optimizer_py import (
    Schedule,
    OptimizerContext,
)

class DeviceController(ABC):
    """
    Abstract base class for device controllers.
    
    Each controller acts as a Facade for its device subsystem,
    hiding the complexity of device communication from the Orchestrator.
    """
    
    @property
    @abstractmethod
    def device_id(self) -> int:
        """Get the ID of the controlled device."""
        pass
    
    @property
    @abstractmethod
    def device_name(self) -> str:
        """Get the name of the controlled device."""
        pass
    
    @abstractmethod
    def use_schedule(self, schedule: Schedule) -> None:
        """
        Inform the controller about the schedule to use.
        
        The controller stores this schedule and uses it when
        updateDevice() is called to determine the device behavior.
        
        Args:
            schedule: The optimized schedule from the optimizer
        """
        pass
    
    @abstractmethod
    def add_to_optimizer_context(self, context: OptimizerContext, current_time: datetime = None) -> None:
        """
        Add device information to the optimizer context.
        
        This adds all relevant information about the device that
        the optimizer needs to create an optimized schedule.
        
        Args:
            context: The optimizer context to add information to
            context_start_time: If provided, the datetime the optimizer considers as start
        """
        pass
    
    @abstractmethod
    def update_device(self, current_time: datetime) -> None:
        """
        Update the physical device based on the current schedule.
        
        This method:
        1. Looks up the behavior for the device at the current time
        2. Instructs the device (via interactor) how to behave
        """
        pass
    
    @abstractmethod
    def get_current_state(self) -> dict:
        """
        Get the current state of the device.
        
        Returns:
            Dictionary containing current device state
        """
        pass
