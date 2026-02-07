from datetime import datetime, timezone
from ..interfaces import BatteryInteractor
from models.devices import Battery

from electricity_price_optimizer_py import units

class MockBatteryInteractor(BatteryInteractor):
    """Mock implementation of battery interactor for testing."""
    
    def __init__(
        self,
        battery_id: int,
    ):
        self._battery_id = battery_id
        self._charge = units.WattHour(0)
        self._current = units.Watt(0)
        self._last_update = datetime.now(timezone.utc)

    def set_current(self, current: units.Watt, device_manager: DeviceManager) -> None:
        """Set the charge/discharge current in W."""
        # Clamp to valid range
        # Work with raw numeric values to avoid relying on ordering for unit wrappers
        battery = device_manager.get_battery(self._battery_id)

        if current > 0:  # Charging: clamp to max_charge_rate
            self._current = min(battery.max_charge_rate, current)
        else:            # Discharging: clamp to max_discharge_rate (negative)
            self._current = max(-battery.max_discharge_rate, current)
    
    def get_charge(self) -> units.WattHour:
        """Get the current charge level in Wh."""
        return units.WattHour(self._charge)
    
    def get_current(self) -> units.Watt:
        """Get the current charge/discharge rate in W."""
        return self._current

    def update(self, current_time: datetime) -> None:
        elapsed = (current_time - self._last_update)
        """Update the battery state based on elapsed time."""
        # Use numeric comparison for unit wrapper
        cur_val = getattr(self._current, "value", float(self._current))
        if abs(cur_val) < 1e-12:
            return

        # Multiply Watt by timedelta -> WattHour (units wrapper implements this)
        energy_change = self._current * elapsed
        
        # Update charge level with clamping
        self._charge =  max(units.WattHour(0),
                            min(self.battery.capacity, self._charge + energy_change)
                            )
        self._last_update = current_time