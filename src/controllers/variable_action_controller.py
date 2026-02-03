from datetime import datetime
from typing import Optional

from .base import DeviceController
from models import VariableAction
from interactors import VariableActionInteractor

from electricity_price_optimizer_py import (
    Schedule,
    OptimizerContext,
    VariableAction as OptimizerVariableAction,
    AssignedVariableAction
)

class VariableActionController(DeviceController):
    """
    Controller for variable action devices (e.g., EV charging).
    
    Manages actions with variable power consumption profiles.
    """
    
    def __init__(self, action: VariableAction, interactor: VariableActionInteractor):
        """
        Initialize the variable action controller.
        
        Args:
            action: The variable action model
            interactor: The interactor for device communication (injected)
        """
        self._action = action
        self._interactor = interactor
        self._schedule: Optional[Schedule] = None
    
    @property
    def device_id(self) -> int:
        return self._action.id
    
    @property
    def device_name(self) -> str:
        return self._action.name
    
    @property
    def action(self) -> VariableAction:
        """Get the action model."""
        return self._action
    
    def use_schedule(self, schedule: Schedule) -> None:
        """Store the schedule for later use."""
        self._schedule = schedule
    
    def add_to_optimizer_context(self, context: OptimizerContext, current_time: datetime) -> None:
        start = self._action.start
        end = self._action.end

        # Clamp start to the optimization horizon start (often "current_time")
        if start < current_time:
            start = current_time

        # If clamping makes the window invalid, skip or handle as unschedulable
        if start >= end:
            return  # or raise ValueError / mark not plannable

        optimizer_action = OptimizerVariableAction(
            id=self._action.id,
            total_consumption=self._action.total_energy_wh,
            max_consumption=self._action.max_power_watts,
            start=start,
            end=end,
        )
        context.add_variable_action(optimizer_action)

    
    def update_device(self, current_time: datetime) -> None:
        """
        Update the device based on the current schedule.
        
        Looks up the power consumption for the current time
        from the schedule and sets the device accordingly.
        """
        if self._schedule is None:
            return
        
        assigned = self._schedule.get_variable_action(self._action.id)
        if assigned is None:
            return
        
        try:
            # Get the consumption rate for the current time
            consumption = assigned.get_consumption(current_time)
            # Instruct the interactor to set this consumption
            self._interactor.set_current(consumption)
        except ValueError:
            # Time is outside schedule range, stop consumption
            self._interactor.set_current(0)
    
    def get_current_state(self) -> dict:
        """Get the current state of the action."""
        return {
            "id": self._action.id,
            "name": self._action.name,
            "type": "variable_action",
            "current_consumption": self._interactor.get_current(),
            "total_consumed": self._interactor.get_total_consumed(),
            "total_required": self._action.total_energy_wh,
            "max_consumption": self._action.max_power_watts,
            # compute percentage using numeric values to avoid unit-wrapper arithmetic
            "progress_percentage": (
                getattr(self._interactor.get_total_consumed(), "value", float(self._interactor.get_total_consumed()))
                / getattr(self._action.total_energy_wh, "value", float(self._action.total_energy_wh))
            ) * 100,
            "time_window": {
                "start": self._action.start.isoformat(),
                "end": self._action.end.isoformat(),
            },
        }
    
    def update_action_model(self, action: VariableAction) -> None:
        """Update the action model with new parameters."""
        self._action = action
