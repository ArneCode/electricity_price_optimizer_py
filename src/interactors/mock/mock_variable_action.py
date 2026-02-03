from datetime import datetime
from ..interfaces import VariableActionInteractor

from models.actions import VariableAction
from electricity_price_optimizer_py import units

class MockVariableActionInteractor(VariableActionInteractor):
    """Mock implementation of variable action interactor for testing."""
    
    def __init__(
        self,
        action: VariableAction,
    ):
        self.action = action
        self._current = units.Watt(0)
        self._total_consumed = units.WattHour(0)
        self._last_update = datetime.now()
        # store limits as unit objects but keep numeric access via .value
        self.max_consumption = units.Watt(action.max_power_watts)
        self.total_required = units.WattHour(action.total_energy_wh)
    
    def set_current(self, current: units.Watt) -> None:
        """Set the power consumption in W."""
        # Clamp to valid range
        # Use numeric values for clamping because unit objects don't support
        # Python's built-in min/max reliably across wrapper types.
        cur_val = getattr(current, "value", float(current))
        max_val = getattr(self.max_consumption, "value", float(self.max_consumption))
        clamped = max(0.0, min(cur_val, max_val))
        self._current = units.Watt(clamped)
    
    def get_current(self) -> units.Watt:
        """Get the current power consumption in W."""
        return self._current
    
    def get_total_consumed(self) -> units.WattHour:
        """Get total energy consumed so far in Wh."""
        return self._total_consumed
    
    def update(self, current_time: datetime) -> None:
        """Update the consumption state based on elapsed time."""
        # compare numeric values to avoid unit-wrapper comparison issues
        if getattr(self._current, "value", 0) != 0:
            # Calculate elapsed time
            elapsed = (current_time - self._last_update)
            
            # Calculate energy consumed in Wh
            energy = self._current * elapsed
            self._total_consumed += energy
            # Stop if we've consumed enough (compare numeric values)
            if getattr(self._total_consumed, "value", float(self._total_consumed)) >= getattr(self.total_required, "value", float(self.total_required)):
                self._current = units.Watt(0)
        self._last_update = current_time
            