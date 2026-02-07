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
        self._last_update = datetime.now(timezone.utc)
        self._initialized = True

    
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
        self._last_update = datetime.now(timezone.utc)
    
    @classmethod
    def get_instance(cls) -> "SmartHomeMock":
        """Get the singleton instance."""
        return cls()
