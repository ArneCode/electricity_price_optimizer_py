from datetime import datetime
from typing import Optional
from ..interfaces import ConstantActionInteractor, ActionState
from models.actions import ConstantAction

from electricity_price_optimizer_py import units

class MockConstantActionInteractor(ConstantActionInteractor):
    """Mock implementation of constant action interactor for testing."""
    
    def __init__(
        self,
        action: ConstantAction,
    ):
        self.action = action
        self._state = ActionState.IDLE
        self._start_time: Optional[datetime] = None
     
    def start_action(self) -> None:
        """Start the action."""
        if self._state == ActionState.IDLE:
            self._state = ActionState.RUNNING
            self._start_time = datetime.now()
    
    def stop_action(self) -> None:
        """Stop the action."""
        self._state = ActionState.IDLE
        self._start_time = None
    
    def get_action_state(self) -> ActionState:
        """Get the current state of the action."""
        return self._state
    
    def get_current(self) -> units.Watt:
        """Get the current power consumption in W."""
        if self._state == ActionState.RUNNING:
            return self.action.power_watts
        return units.Watt(0)
    
    def get_start_time(self) -> datetime:
        """Get the action start time."""
        return self._start_time
    
    def update(self, current_time: datetime) -> None:
        """Update action state based on current time."""
        if self._state == ActionState.RUNNING and self._start_time:
            elapsed = (current_time - self._start_time).total_seconds() / 60
            if elapsed >= self.action.duration_minutes:
                self._state = ActionState.COMPLETED
