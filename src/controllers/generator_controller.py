from datetime import datetime
from typing import Optional, List

from .base import DeviceController
from interactors import GeneratorInteractor

from electricity_price_optimizer_py import (
    Schedule,
    OptimizerContext,
    Prognoses
)


class GeneratorController(DeviceController):
    """
    Controller for generator devices (e.g., PV panels).
    
    Generators are not directly controlled but their output is
    read and used in optimization.
    """
    
    def __init__(self, generator: Generator, interactor: GeneratorInteractor):
        """
        Initialize the generator controller.
        
        Args:
            generator: The generator device model
            interactor: The interactor for device communication (injected)
        """
        self._generator = generator
        self._interactor = interactor
        self._schedule: Optional[Schedule] = None
    
    @property
    def device_id(self) -> str:
        return self._generator.id
    
    @property
    def device_name(self) -> str:
        return self._generator.name
    
    @property
    def generator(self) -> Generator:
        """Get the generator model."""
        return self._generator
    
    def set_generation_prognosis(self, prognosis: Prognoses) -> None:
        """
        Set the generation prognosis from weather data.
        
        This is typically called by the DeviceManager after
        fetching weather data from the Weather API.
        """
        self._generation_prognosis = prognosis
    
    def use_schedule(self, schedule: Schedule) -> None:
        """Store the schedule (generators don't actively use it)."""
        self._schedule = schedule
    
    def add_to_optimizer_context(self, context: OptimizerContext) -> None:
        """
        Add generator information to the optimizer context.
        
        Adds the generation prognosis to the context's generated electricity.
        """
        if self._generation_prognosis is not None:
            # Combine with existing generation prognosis
            if context.generated_electricity is not None:
                # Add this generator's output to the total
                for i in range(len(context.generated_electricity.values)):
                    if i < len(self._generation_prognosis.values):
                        context.generated_electricity.values[i] += self._generation_prognosis.values[i]
            else:
                context.generated_electricity = self._generation_prognosis
    
    def update_device(self) -> None:
        """
        Update device state.
        
        Generators don't need active control, but we can read
        their current output for monitoring.
        """
        # Generators are passive - nothing to update
        pass
    
    def get_current_state(self) -> dict:
        """Get the current state of the generator."""
        return {
            "id": self._generator.id,
            "name": self._generator.name,
            "type": "generator",
            "current_output": self._interactor.get_current(),
            "max_power": self._generator.max_power,
            "location": {
                "latitude": self._generator.latitude,
                "longitude": self._generator.longitude,
            },
        }
    
    def update_generator_model(self, generator: Generator) -> None:
        """Update the generator model with new parameters."""
        self._generator = generator
