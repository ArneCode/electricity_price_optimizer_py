from datetime import datetime
from typing import Optional, TYPE_CHECKING

from .base import DeviceController

from electricity_price_optimizer_py import (
    Schedule,
    OptimizerContext,
    VariableAction as OptimizerVariableAction,
    AssignedVariableAction
)

if TYPE_CHECKING:
    from device_manager import IDeviceManager


class VariableActionController(DeviceController):
    """
    Controller for variable action devices (e.g., EV charging).

    Manages actions with variable power consumption profiles.
    """

    def __init__(self, id: "int"):
        self._id = id
        self._schedule: "Optional[Schedule]" = None

    @property
    def device_id(self) -> "int":
        return self._id

    def use_schedule(self, schedule: "Schedule", device_manager: "IDeviceManager") -> "None":
        """Store the schedule for later use."""
        self._schedule = schedule

    def add_to_optimizer_context(self, context: "OptimizerContext", current_time: "datetime", device_manager: "IDeviceManager") -> "None":
        action = device_manager.get_device_service(
        ).get_variable_action_device(self._id).actions[0]
        interactor = device_manager.get_interactor_service(
        ).get_variable_action_interactor(self._id)

        if interactor.get_total_consumed(device_manager) >= action.total_consumption:
            return  # already fully consumed, no need to add to context

        if action is None:
            return

        start = action.start
        end = action.end

        # Clamp start to the optimization horizon start (often "current_time")
        if start < current_time:
            start = current_time

        # If clamping makes the window invalid, skip or handle as unschedulable
        if start >= end:
            return  # or raise ValueError / mark not plannable

        optimizer_action = OptimizerVariableAction(
            start=start,
            end=end,
            total_consumption=action.total_consumption,
            max_consumption=action.max_consumption,
            id=self._id,  # maybe should be action ID instead of device ID
        )
        context.add_variable_action(optimizer_action)

    def update_device(self, current_time: "datetime", device_manager: "IDeviceManager") -> "None":
        """
        Update the device based on the current schedule.

        Looks up the power consumption for the current time
        from the schedule and sets the device accordingly.
        """
        if self._schedule is None:
            return

        assigned = self._schedule.get_variable_action(self._id)
        interactor = device_manager.get_interactor_service(
        ).get_variable_action_interactor(self._id)

        if assigned is None:
            return

        try:
            # Get the consumption rate for the current time
            consumption = assigned.get_consumption(current_time)
            # Instruct the interactor to set this consumption
            interactor.set_current(consumption, device_manager)
        except ValueError:
            # Time is outside schedule range, stop consumption
            interactor.set_current(0)
