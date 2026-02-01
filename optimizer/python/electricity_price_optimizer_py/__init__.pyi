from datetime import datetime, timedelta
from typing import Callable, Optional, Tuple
from typing import Generic, TypeVar
from . import units

T = TypeVar('T')


class PrognosesProvider(Generic[T]):
    """Provides prognosis data via a callback function."""

    def __init__(self, get_data: Callable[[datetime, datetime], T]) -> None:
        """
        Initialize the provider.

        Args:
            get_data: A function that takes a (start_time, end_time) and returns 
                      the predicted value for that interval.
        """
        ...


class ConstantAction:
    """An action with a fixed duration and constant consumption rate."""
    start_from: datetime
    end_before: datetime
    duration: timedelta
    consumption: units.Watt

    def __init__(
        self,
        start_from: datetime,
        end_before: datetime,
        duration: timedelta,
        consumption: units.Watt,
        id: int,
    ) -> None:
        """
        Args:
            start_from: The earliest time the action can start.
            end_before: The latest time the action must end before.
            duration: The duration of the action (must be < 1 day and a multiple of the timestep).
            consumption: The fixed consumption amount for every timestep.
            id: Unique identifier for the action.
        """
        ...


class AssignedConstantAction:
    """An instance of a ConstantAction that has been scheduled at a specific time."""

    def get_start_time(self) -> datetime:
        """Returns the scheduled start time."""
        ...

    def get_end_time(self) -> datetime:
        """Returns the scheduled end time."""
        ...

    def get_id(self) -> int:
        """Returns the action's unique identifier."""
        ...


class VariableAction:
    """An action where consumption can be spread flexibly over a time window."""
    start: datetime
    end: datetime
    total_consumption: units.WattHour
    max_consumption: units.Watt

    def __init__(
        self,
        start: datetime,
        end: datetime,
        total_consumption: units.WattHour,
        max_consumption: units.Watt,
        id: int,
    ) -> None:
        """
        Args:
            start: The earliest time the action can start.
            end: The latest time the action must end.
            total_consumption: The total energy consumption required.
            max_consumption: The maximum consumption allowed in a single timestep.
            id: Unique identifier for the action.
        """
        ...


class AssignedVariableAction:
    """An instance of a VariableAction with consumption allocated across the schedule."""

    def get_consumption(self, time: datetime) -> units.Watt:
        """Returns the allocated consumption for the specific timestep."""
        ...

    def get_id(self) -> int:
        """Returns the action's unique identifier."""
        ...


class Battery:
    """Represents a physical battery for energy storage."""
    capacity: units.WattHour
    max_charge_rate: units.Watt
    max_discharge_rate: units.Watt
    initial_charge: units.WattHour

    def __init__(
        self,
        capacity: units.WattHour,
        max_charge_rate: units.Watt,
        max_discharge_rate: units.Watt,
        initial_charge: units.WattHour,
        id: int,
    ) -> None:
        """
        Args:
            capacity: Total energy storage capacity.
            max_charge_rate: Maximum units of energy added per timestep.
            max_discharge_rate: Maximum units of energy removed per timestep.
            initial_charge: Starting energy level.
            id: Unique identifier for the battery.
        """
        ...


class AssignedBattery:
    """A battery's state over the course of a schedule."""

    def get_charge_level(self, time: datetime) -> units.WattHour:
        """Returns the battery charge level at the given time."""
        ...

    def get_charge_speed(self, time: datetime) -> units.Watt:
        """Returns the battery charge/discharge speed at the given time."""
        ...

    def get_id(self) -> int:
        """Returns the battery's unique identifier."""
        ...


class OptimizerContext:
    """The environment and constraints used to run the price optimization."""

    def __init__(
        self,
        time: datetime,
        electricity_price: PrognosesProvider[units.EuroPerWh],
    ) -> None:
        """
        Initialize the context.

        Args:
            time: The start time for the optimization period.
            electricity_price: Provider for the electricity price forecast.
        """
        ...

    def add_constant_action(self, action: ConstantAction) -> None:
        """Adds a new constant action to be scheduled."""
        ...

    def add_variable_action(self, action: VariableAction) -> None:
        """Adds a new variable action to be scheduled."""
        ...

    def add_battery(self, battery: Battery) -> None:
        """Adds a battery to be utilized in the optimization."""
        ...

    def add_past_constant_action(self, action: AssignedConstantAction) -> None:
        """Adds an action already in progress to the fixed consumption base."""
        ...

    def add_generated_electricity_prognoses(self, provider: PrognosesProvider[units.WattHour]) -> None:
        """Adds predicted energy generation (e.g., Solar) to the context."""
        ...


class Schedule:
    """The result of an optimization run containing assigned actions and battery states."""

    def get_constant_action(self, id: int) -> Optional[AssignedConstantAction]:
        """Retrieve a specific scheduled constant action by ID."""
        ...

    def get_variable_action(self, id: int) -> Optional[AssignedVariableAction]:
        """Retrieve a specific scheduled variable action by ID."""
        ...

    def get_battery(self, id: int) -> Optional[AssignedBattery]:
        """Retrieve the state of a specific battery by ID."""
        ...


def run_simulated_annealing(context: OptimizerContext) -> Tuple[units.Euro, Schedule]:
    """
    Runs the simulated annealing optimization algorithm.

    Args:
        context: The optimization context containing prices, actions, and batteries.

    Returns:
        A tuple of (total_cost, optimized_schedule).
    """
    ...
