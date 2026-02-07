from datetime import datetime, timedelta
from typing import Optional

from .base import DeviceController
from interactors import ConstantActionInteractor
from interactors.interfaces import ActionState

from electricity_price_optimizer_py import (
    Schedule,
    OptimizerContext,
    ConstantAction as OptimizerConstantAction,
)

from device_manager import IDeviceManager

class ConstantActionController(DeviceController):
    
    def __init__(
            self, 
            id: int,
        ):
        self._id = id
        self._schedule: Optional[Schedule] = None
    
    @property
    def id(self) -> int:
        return self._id
    
    @property
    def assigned_start_time(self) -> Optional[datetime]:
        if self._schedule is None:
            return None
        assigned = self._schedule.get_constant_action(self._id)
        if assigned is None:
            return None
        # AssignedConstantAction from the Rust wrapper exposes accessors
        return assigned.get_start_time()
    
    def is_controllable(self, device_manager: IDeviceManager) -> bool:
        interactor = device_manager.get_interactor_service().get_constant_action_interactor(self._id)
        state = interactor.get_action_state(device_manager)
        return state in (ActionState.IDLE, ActionState.COMPLETED)

    def use_schedule(self, schedule: Schedule, device_manager: IDeviceManager) -> None:
        if self.is_controllable(device_manager):
            self._schedule = schedule

    def add_to_optimizer_context(self, context: OptimizerContext, current_time: datetime, device_manager: IDeviceManager) -> None:
        action = device_manager.get_device_service().get_constant_action_device(self._id).actions[0]
        if action is None:
            return
        
        if self.is_controllable(device_manager):
            
            # Clamp to optimizer horizon start (typically current_time)
            start_from = max(action.start_from, current_time)
            end_before = action.end_before

            # If the window is impossible, don't add it (or handle differently)
            if end_before is None or start_from >= end_before:
                return

            context.add_constant_action(
                OptimizerConstantAction(
                    start_from=start_from,
                    end_before=end_before,
                    duration=action.duration,
                    consumption=action.consumption,
                    id=self._id, # maybe needs to be action ID
                )
            )
        else:
            if self._schedule is None:
                return
            assigned = self._schedule.get_constant_action(self._id)
            if assigned is not None:
                context.add_past_constant_action(assigned)

    
    def update_device(self, current_time: datetime, device_manager: IDeviceManager) -> None:
        interactor = device_manager.get_interactor_service().get_constant_action_interactor(self._id)
        state = interactor.get_action_state(device_manager)
        
        if state == ActionState.IDLE:
            assigned_start = self.assigned_start_time
            if assigned_start and current_time >= assigned_start:
                interactor.start_action(device_manager)
