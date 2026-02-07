from abc import ABC, abstractmethod
from typing import TYPE_CHECKING
from electricity_price_optimizer_py import Schedule, OptimizerContext, PrognosesProvider, run_simulated_annealing
from electricity_price_optimizer_py.units import EuroPerWh
from datetime import datetime, timezone

if TYPE_CHECKING:
    from device_manager import IDeviceManager


class IOrchestratorService(ABC):
    """Orchestrator service interface providing access to all services."""

    @abstractmethod
    def get_schedule(self) -> "Schedule":
        """Get the current schedule."""
        ...

    @abstractmethod
    def run_optimization(self, device_manager: "IDeviceManager") -> "None":
        """Run the optimization algorithm."""
        ...


class OrchestratorService(IOrchestratorService):
    _schedule: "Schedule | None"

    def __init__(self):
        self._schedule = None

    def get_schedule(self) -> "Schedule":
        """Get the current schedule."""
        if self._schedule is None:
            raise ValueError("Schedule has not been generated yet.")
        return self._schedule

    def run_optimization(self, device_manager: "IDeviceManager") -> "None":
        """Run the optimization algorithm."""
        now = datetime.now(timezone.utc)

        # Create a simple context with mock price data for demonstration
        price_provider = PrognosesProvider(
            lambda t1, t2: EuroPerWh(0.20)  # Mock constant price of 0.20 €/Wh
        )
        context = OptimizerContext(
            time=now,
            electricity_price=price_provider
        )

        # Add devices and actions from the device manager to the context
        for controller in device_manager.get_controller_service().get_all_controllers():
            controller.add_to_optimizer_context(context, now, device_manager)

        # Run the optimization algorithm
        cost, schedule = run_simulated_annealing(context)
        print(f"Optimization completed with total cost: {cost}")
        self._schedule = schedule
        for controller in device_manager.get_controller_service().get_all_controllers():
            controller.use_schedule(schedule, device_manager)
