from datetime import datetime
import math
from ..interfaces import GeneratorInteractor
from models.devices import Generator

from electricity_price_optimizer_py import units

class MockGeneratorInteractor(GeneratorInteractor):
    """Mock implementation of generator interactor for testing."""
    
    def __init__(
        self,
        generator: Generator,
        max_power: units.Watt,
        latitude: float,
        longitude: float,
    ):
        self.generator = generator
        self.max_power = max_power
        self.latitude = latitude
        self.longitude = longitude
        self._current_power = units.Watt(0)
    
    def get_current(self) -> units.Watt:
        """Get the current power generation in W."""
        return self._current_power
    
    def set_simulated_power(self, power: units.Watt) -> None:
        """Set the simulated power output (for testing)."""
        self._current_power = min(power, self.max_power)
    
    def update_from_weather(self, cloud_cover: float, time_of_day: datetime) -> None:
        """
        Update power output based on weather and time.
        
        Args:
            cloud_cover: 0.0 (clear) to 1.0 (overcast)
            time_of_day: Current time for solar angle calculation
        """
        hour = time_of_day.hour + time_of_day.minute / 60
        
        # Simple solar model: peak at noon, zero at night
        if 6 <= hour <= 20:
            # Sinusoidal model for daylight hours
            solar_factor = math.sin(math.pi * (hour - 6) / 14)
            solar_factor = max(0, solar_factor)
        else:
            solar_factor = 0
        
        # Apply cloud cover reduction
        weather_factor = 1 - (cloud_cover * 0.8)  # Clouds reduce output by up to 80%
        
        self._current_power = units.Watt(self.max_power.value * solar_factor * weather_factor)
