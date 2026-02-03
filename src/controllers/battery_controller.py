from datetime import datetime, timezone
from typing import Optional

from .base import DeviceController
from interactors import BatteryInteractor

from models.devices import Battery

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
    
    def __init__(self, battery: Battery, interactor: BatteryInteractor):
        """
        Initialize the battery controller.
        
        Args:
            battery: The battery device model
            interactor: The interactor for device communication (injected)
        """
        self._battery = battery
        self._interactor = interactor
        self._schedule: Optional[Schedule] = None
    
    @property
    def device_id(self) -> int:
        return self._battery.id
    
    @property
    def device_name(self) -> str:
        return self._battery.name
    
    @property
    def battery(self) -> Battery:
        """Get the battery model."""
        return self._battery
    
    def use_schedule(self, schedule: Schedule) -> None:
        """Store the schedule for later use when updating the device."""
        self._schedule = schedule
    
    def add_to_optimizer_context(self, context: OptimizerContext, current_time: datetime) -> None:
        """
        Add battery information to the optimizer context.
        
        Updates the battery's initial level from the actual current charge
        before adding to context.
        """
        # Update initial level from actual device state
        current_charge = self._interactor.get_charge()

        optimizer_battery = OptimizerBattery(
            capacity=self._battery.capacity,
            max_charge_rate=self._battery.maximum_charge_rate,
            max_discharge_rate=self._battery.maximum_output_rate,
            initial_charge=current_charge,
            id=self._battery.id,
        )
        context.add_battery(optimizer_battery)
    
    def update_device(self, current_time: datetime) -> None:
        """
        Update the battery based on the current schedule.
        
        Looks up the charge rate for the current time from the schedule
        and instructs the battery to charge/discharge at that rate.
        """
        if self._schedule is None:
            return
        
        # `Schedule` wrapper exposes `get_battery(id)` which returns an AssignedBattery
        assigned = self._schedule.get_battery(self._battery.id)
        if assigned is None:
            return

        try:
            # Get the charge speed (W) for the current time from the AssignedBattery
            charge_rate = assigned.get_charge_speed(current_time)
            # Instruct the battery interactor to set this charge rate (expects units.Watt)
            self._interactor.set_current(charge_rate)
        except ValueError:
            # Time is outside schedule range, do nothing
            pass
    
    def get_current_state(self) -> dict:
        """Get the current state of the battery."""
        return {
            "id": self._battery.id,
            "name": self._battery.name,
            "type": "battery",
            "charge": self._interactor.get_charge(),
            "current": self._interactor.get_current(),
            "capacity": self._battery.capacity,
            "charge_percentage": (
                (getattr(self._interactor.get_charge(), "value", float(self._interactor.get_charge())) /
                 getattr(self._battery.capacity, "value", float(self._battery.capacity))) * 100
            ),
        }
    
    def update_battery_model(self, battery: Battery) -> None:
        """Update the battery model with new parameters."""
        self._battery = battery
