from datetime import datetime
from typing import Optional, TYPE_CHECKING

from .base import DeviceController

from electricity_price_optimizer_py import (
    Schedule,
    OptimizerContext,
    PrognosesProvider
)

if TYPE_CHECKING:
    from device_manager import IDeviceManager


class GeneratorController(DeviceController):
    """Controller for generator devices (e.g., PV panels).

    Generators are passive: they don't receive commands but their
    prognoses/current output is added to the optimizer context.
    """

    def __init__(self, id: "int"):
        self._id = id
        self._schedule: "Optional[Schedule]" = None
        self._generation_prognosis: "Optional[Prognoses]" = None

    @property
    def id(self) -> "int":
        return self._id

    def use_schedule(self, schedule: "Schedule", device_manager: "IDeviceManager") -> "None":
        """Store the schedule (generators typically don't act on it)."""
        self._schedule = schedule

    def set_generation_prognosis(self, prognosis: "Prognoses", device_manager: "IDeviceManager") -> "None":
        """Store the generation prognosis for later addition to optimizer context."""
        self._generation_prognosis = prognosis

    def add_to_optimizer_context(self, context: "OptimizerContext", current_time: "datetime", device_manager: "IDeviceManager") -> "None":
        """Add generator output (prognosis) into the optimizer context.

        This will sum this generator's prognosis into context.generated_electricity.
        """
        if self._generation_prognosis is None:
            # Try to read a current value from interactor and add as a single-point prognosis
            interactor = device_manager.get_interactor_service().get_generator_interactor(self._id)
            if interactor is None:
                return
            current = interactor.get_current(device_manager)
            # create a very small Prognoses-like object if available else skip
            try:
                # try to append into context if the structure exists
                if context.generated_electricity is None:
                    context.generated_electricity = self._generation_prognosis
            except Exception:
                pass
            return

        # Combine with existing generation prognosis
        if context.generated_electricity is not None:
            for i in range(len(context.generated_electricity.values)):
                if i < len(self._generation_prognosis.values):
                    context.generated_electricity.values[i] += self._generation_prognosis.values[i]
        else:
            context.generated_electricity = self._generation_prognosis

    def update_device(self, current_time: "datetime", device_manager: "IDeviceManager") -> "None":
        """Optional periodic update; for generators we generally don't actuate devices."""
        # Could poll interactor to advance simulated state if it exposes update()
        interactor = device_manager.get_interactor_service().get_generator_interactor(self._id)
        if interactor is None:
            return
        # Some mock interactors implement update(current_time, device_manager)
        try:
            interactor.update(current_time, device_manager)
        except Exception:
            # Not all interactors implement update; ignore quietly
            pass
