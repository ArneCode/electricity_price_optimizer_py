from datetime import datetime, timezone
from typing import Optional

from backend.src.device_manager import IDeviceManager

from .base import DeviceController
from interactors import BatteryInteractor

from electricity_price_optimizer_py import (
    Schedule,
    OptimizerContext,
    Battery as OptimizerBattery
)

class BatteryController(DeviceController):
    """
    Controller for battery devices.
    
    Acts as a Facade that hides:
    - Interactor communication
    - Schedule data conversion
    - Device control logic
    """
    
    def __init__(
            self,
            id: int, 
        ):

        self._id = id
        self._schedule: Optional[Schedule] = None
    
    @property
    def device_id(self) -> int:
        return self.id
    
    def use_schedule(self, schedule: Schedule, device_manager: IDeviceManager) -> None:
        """Store the schedule for later use when updating the device."""
        self._schedule = schedule
    
    def add_to_optimizer_context(self, context: OptimizerContext, current_time: datetime, device_manager: IDeviceManager) -> None:
        """
        Add battery information to the optimizer context.
        
        Updates the battery's initial level from the actual current charge
        before adding to context.
        """

        # Update initial level from actual device state
        battery_interactor = device_manager.get_interactor_service().get_battery_interactor(self._id)
        battery = device_manager.get_device_service().get_battery(self._id)

        current_charge = battery_interactor.get_charge(device_manager)

        optimizer_battery = OptimizerBattery(
            capacity=battery.capacity,
            max_charge_rate=battery.max_charge_rate,
            max_discharge_rate=battery.max_discharge_rate,
            initial_charge=current_charge,
            id=self._id,
        )
        context.add_battery(optimizer_battery)
    
    def update_device(self, current_time: datetime, device_manager: IDeviceManager) -> None:
        """
        Update the battery based on the current schedule.
        
        Looks up the charge rate for the current time from the schedule
        and instructs the battery to charge/discharge at that rate.
        """
        if self._schedule is None:
            return
        
        # `Schedule` wrapper exposes `get_battery(id)` which returns an AssignedBattery
        assigned = self._schedule.get_battery(self._id)
        if assigned is None:
            return

        try:
            # Get the charge speed (W) for the current time from the AssignedBattery
            charge_rate = assigned.get_charge_speed(current_time)
            # Instruct the battery interactor to set this charge rate (expects units.Watt)
            interactor = device_manager.get_interactor_service().get_battery_interactor(self._id)
            interactor.set_current(charge_rate, device_manager)
        except ValueError:
            # Time is outside schedule range, do nothing
            print(f"Current time {current_time} is outside the schedule range for battery {self._id}. No update applied.")
            pass