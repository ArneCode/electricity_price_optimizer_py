from datetime import datetime
from typing import Optional

from backend.src.device_manager import DeviceManager
from ..interfaces import ConstantActionInteractor, ActionState
from models.actions import ConstantAction

from electricity_price_optimizer_py import units

from device_manager import IDeviceManager

class MockConstantActionInteractor(ConstantActionInteractor):
    """Mock implementation of constant action interactor for testing."""
    
    def __init__(
        self,
        device_id: int,
    ):
        self._device_id = device_id
        self._state = ActionState.IDLE
        self._start_time: Optional[datetime] = None
     
    def start_action(self, device_manager: IDeviceManager) -> None:
        """Start the action."""
        if self._state == ActionState.IDLE:
            self._state = ActionState.RUNNING
            self._start_time = datetime.now()
    
    def stop_action(self, device_manager: IDeviceManager) -> None:
        """Stop the action."""
        self._state = ActionState.IDLE
        self._start_time = None
    
    def get_action_state(self, device_manager: IDeviceManager) -> ActionState:
        """Get the current state of the action."""
        return self._state
    
    def get_current(self, device_manager: IDeviceManager) -> units.Watt:
        """Get the current power consumption in W."""
        if self._state == ActionState.RUNNING:
            action = device_manager.get_device_service().get_constant_action_device(self._device_id).actions[0]
            return action.consumption
        return units.Watt(0)
    
    def get_start_time(self, device_manager: IDeviceManager) -> datetime:
        """Get the action start time."""
        return self._start_time
    
    def update(self, current_time: datetime, device_manager: IDeviceManager) -> None:
        """Update action state based on current time."""
        if self._state == ActionState.RUNNING and self._start_time:
            action = device_manager.get_device_service().get_constant_action_device(self._device_id).actions[0]
            elapsed = (current_time - self._start_time)
            if elapsed >= action.duration:
                self._state = ActionState.COMPLETED
