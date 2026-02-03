from datetime import datetime, timezone
from typing import Dict
from .mock_battery import MockBatteryInteractor
from .mock_generator import MockGeneratorInteractor
from .mock_constant_action import MockConstantActionInteractor
from .mock_variable_action import MockVariableActionInteractor



class SmartHomeMock:
    """
    Singleton that manages all mock devices and simulates their behavior.
    
    This class simulates real device behavior over time, updating states
    based on elapsed time since last update.
    """
    
    _instance = None
    
    def __new__(cls):
        if cls._instance is None:
            cls._instance = super().__new__(cls)
            cls._instance._initialized = False
        return cls._instance
    
    def __init__(self):
        if self._initialized:
            return

        self._batteries: Dict[int, MockBatteryInteractor] = {}
        self._generators: Dict[int, MockGeneratorInteractor] = {}
        self._constant_actions: Dict[int, MockConstantActionInteractor] = {}
        self._variable_actions: Dict[int, MockVariableActionInteractor] = {}
        self._last_update = datetime.now(timezone.utc)
        self._initialized = True
    
    def add_battery(self, battery: MockBatteryInteractor) -> None:
        """Add a mock battery to the simulation."""
        self._batteries[battery.battery.id] = battery
    
    def add_generator(self, generator: MockGeneratorInteractor) -> None:
        """Add a mock generator to the simulation."""
        self._generators[generator.generator.id] = generator
    
    def add_constant_action(self, action: MockConstantActionInteractor) -> None:
        """Add a mock constant action to the simulation."""
        self._constant_actions[action.action.id] = action
    
    def add_variable_action(self, action: MockVariableActionInteractor) -> None:
        """Add a mock variable action to the simulation."""
        self._variable_actions[action.action.id] = action
    
    def remove_device(self, device_id: int) -> bool:
        """Remove a device from the simulation."""
        if device_id in self._batteries:
            del self._batteries[device_id]
            return True
        if device_id in self._generators:
            del self._generators[device_id]
            return True
        if device_id in self._constant_actions:
            del self._constant_actions[device_id]
            return True
        if device_id in self._variable_actions:
            del self._variable_actions[device_id]
            return True
        return False
    
    def get_battery(self, battery_id: int) -> MockBatteryInteractor | None:
        """Get a battery by ID."""
        return self._batteries.get(battery_id)
    
    def get_generator(self, generator_id: int) -> MockGeneratorInteractor | None:
        """Get a generator by ID."""
        return self._generators.get(generator_id)
    
    def get_constant_action(self, action_id: int) -> MockConstantActionInteractor | None:
        """Get a constant action by ID."""
        return self._constant_actions.get(action_id)
    
    def get_variable_action(self, action_id: int) -> MockVariableActionInteractor | None:
        """Get a variable action by ID."""
        return self._variable_actions.get(action_id)
    
    def update_mock_devices(self, current_time: datetime) -> None:
        """
        Update all mock devices based on elapsed time.
        
        This simulates real-world behavior where devices change state
        over time based on their current settings.
        """
        # Update batteries
        for battery in self._batteries.values():
            battery.update(current_time)
        
        # Update constant actions
        for action in self._constant_actions.values():
            action.update(current_time)
        
        # Update variable actions
        for action in self._variable_actions.values():
            action.update(current_time)
        
    
    def reset(self) -> None:
        """Reset the smart home mock to initial state."""
        self._batteries.clear()
        self._generators.clear()
        self._constant_actions.clear()
        self._variable_actions.clear()
        self._last_update = datetime.now(timezone.utc)
    
    @classmethod
    def get_instance(cls) -> "SmartHomeMock":
        """Get the singleton instance."""
        return cls()
