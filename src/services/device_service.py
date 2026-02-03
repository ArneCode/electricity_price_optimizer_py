from abc import ABC, abstractmethod
from sqlalchemy.orm import Session

from device import Device


class IDeviceService(ABC):
    @abstractmethod
    def get_device(self, device_id: int) -> Device | None:
        """Retrieve device details by ID."""
        ...

    @abstractmethod
    def add_device(self, device: Device) -> int:
        """Add a new device and return its ID."""
        ...

    @abstractmethod
    def remove_device(self, device_id: int) -> None:
        """Remove a device by ID."""
        ...


class SqlAlchemyDeviceService(IDeviceService):
    def __init__(self, session: Session):
        self.session = session

    def get_device(self, device_id: int) -> Device | None:
        return self.session.get(Device, device_id)

    def add_device(self, device: Device) -> int:
        self.session.add(device)
        self.session.flush()  # Ensure the ID is generated
        return device.id

    def remove_device(self, device_id: int) -> None:
        device = self.get_device(device_id)
        if device:
            self.session.delete(device)
