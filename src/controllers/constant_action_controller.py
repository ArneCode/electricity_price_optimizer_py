from datetime import datetime, timedelta
from typing import Optional

from .base import DeviceController
from models import ConstantAction
from interactors import ConstantActionInteractor
from interactors.interfaces import ActionState

from electricity_price_optimizer_py import (
    Schedule,
    OptimizerContext,
    ConstantAction as OptimizerConstantAction,
    AssignedConstantAction
)


class ConstantActionController(DeviceController):
    
    def __init__(self, action: ConstantAction, interactor: ConstantActionInteractor):
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
    def action(self) -> ConstantAction:
        return self._action
    
    @property
    def is_controllable(self) -> bool:
        state = self._interactor.get_action_state()
        return state in (ActionState.IDLE, ActionState.COMPLETED)
    
    @property
    def assigned_start_time(self) -> Optional[datetime]:
        if self._schedule is None:
            return None
        assigned = self._schedule.get_constant_action(self._action.id)
        if assigned is None:
            return None
        # AssignedConstantAction from the Rust wrapper exposes accessors
        return assigned.get_start_time()
    
    @property
    def expected_end_time(self) -> Optional[datetime]:
        """Calculate when the running action should complete."""
        start_time = self._interactor.get_start_time()
        if start_time is None:
            return None
        return start_time + self._action.duration_minutes

    @property
    def remaining_duration(self) -> Optional[timedelta]:
        """Time remaining until action completes."""
        end_time = self.expected_end_time
        if end_time is None:
            return None
        remaining = end_time - datetime.now()
        return max(remaining, timedelta(0))

    def use_schedule(self, schedule: Schedule) -> None:
        if self.is_controllable:
            self._schedule = schedule

    def add_to_optimizer_context(self, context: OptimizerContext, current_time: datetime) -> None:
        if self._action is None:
            return

        if self.is_controllable:
            # Clamp to optimizer horizon start (typically current_time)
            start_from = max(self._action.earliest_start, current_time)
            end_before = self._action.latest_end

            # If the window is impossible, don't add it (or handle differently)
            if end_before is None or start_from >= end_before:
                return

            context.add_constant_action(
                OptimizerConstantAction(
                    id=self._action.id,
                    consumption=self._action.power_watts,
                    duration=self._action.duration_minutes,
                    start_from=start_from,
                    end_before=end_before,
                )
            )
        else:
            if self._schedule is None:
                return
            assigned = self._schedule.get_constant_action(self._action.id)
            if assigned is not None:
                context.add_past_constant_action(assigned)

    
    def update_device(self, current_time: datetime) -> None:
        state = self._interactor.get_action_state()
        
        if state == ActionState.IDLE:
            assigned_start = self.assigned_start_time
            if assigned_start and current_time >= assigned_start:
                self._interactor.start_action()
    
    def get_current_state(self) -> dict:
        state = self._interactor.get_action_state()
        start_time = self._interactor.get_start_time()
        return {
            "id": self._action.id,
            "name": self._action.name,
            "type": "constant_action",
            "state": state.value,
            "is_controllable": self.is_controllable,
            "current_consumption": self._interactor.get_current(),
                "consumption": self._action.power_watts,
                "duration": self._action.duration_minutes,
            "started_at": start_time,
            "expected_end_time": self.expected_end_time,
            "remaining_duration": self.remaining_duration,
            "assigned_start_time": self.assigned_start_time,
                "time_window": {
                    "start": self._action.earliest_start,
                    "end": self._action.latest_end,
                },
        }
    
    def update_action_model(self, action: ConstantAction) -> None:
        self._action = action
