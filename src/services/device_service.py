"""Device service layer (SQLAlchemy-backed).

Provides read/write access to Device and specialized tables (Battery, Generator,
ConstantActionDevice, VariableActionDevice) via an injected SQLAlchemy Session.

Notes:
- add_device flushes the session to ensure an auto-generated ID is available.
- remove_device stages deletion; commit outside to persist.
"""
from abc import ABC, abstractmethod
from sqlalchemy.orm import Session
from sqlalchemy import select
from typing import TYPE_CHECKING
if TYPE_CHECKING:
    from device import Device, Battery, Generator, ConstantActionDevice, VariableActionDevice


class IDeviceServiceReader(ABC):
    """Read-only device service API."""
    @abstractmethod
    def get_device(self, device_id: "int") -> "Device | None":
        """Retrieve device details by ID."""
        ...

    @abstractmethod
    def get_battery(self, device_id: "int") -> "Battery | None":
        """Retrieve battery details by ID."""
        ...

    @abstractmethod
    def get_generator(self, device_id: "int") -> "Generator | None":
        """Retrieve generator details by ID."""
        ...

    @abstractmethod
    def get_constant_action_device(self, device_id: "int") -> "ConstantActionDevice | None":
        """Retrieve constant action device details by ID."""
        ...

    @abstractmethod
    def get_variable_action_device(self, device_id: "int") -> "VariableActionDevice | None":
        """Retrieve variable action device details by ID."""
        ...

    @abstractmethod
    def get_all_devices(self) -> "list[Device]":
        """Retrieve all devices."""
        ...

    @abstractmethod
    def get_all_batteries(self) -> "list[Battery]":
        """Retrieve all batteries."""
        ...

    @abstractmethod
    def get_all_generators(self) -> "list[Generator]":
        """Retrieve all generators."""
        ...

    @abstractmethod
    def get_all_constant_action_devices(self) -> "list[ConstantActionDevice]":
        """Retrieve all constant action devices."""
        ...

    @abstractmethod
    def get_all_variable_action_devices(self) -> "list[VariableActionDevice]":
        """Retrieve all variable action devices."""
        ...

    # returns all device ids
    @abstractmethod
    def get_all_device_ids(self) -> "list[int]":
        """Retrieve all device IDs."""
        ...

    @abstractmethod
    def get_all_battery_ids(self) -> "list[int]":
        """Retrieve all battery IDs."""
        ...

    @abstractmethod
    def get_all_generator_ids(self) -> "list[int]":
        """Retrieve all generator IDs."""
        ...

    @abstractmethod
    def get_all_constant_action_device_ids(self) -> "list[int]":
        """Retrieve all constant action device IDs."""
        ...

    @abstractmethod
    def get_all_variable_action_device_ids(self) -> "list[int]":
        """Retrieve all variable action device IDs."""
        ...


class IDeviceService(ABC, IDeviceServiceReader):
    """Device service API with mutation operations."""
    @abstractmethod
    def add_device(self, device: "Device") -> "int":
        """Add a new device and return its ID."""
        ...

    @abstractmethod
    def remove_device(self, device_id: "int") -> "None":
        """Remove a device by ID."""
        ...


class SqlAlchemyDeviceService(IDeviceService):
    """SQLAlchemy-backed implementation of the device service."""

    def __init__(self, session: "Session"):
        self.session = session

    def get_device(self, device_id: "int") -> "Device | None":
        return self.session.get(Device, device_id)

    def get_battery(self, device_id: "int") -> "Battery | None":
        return self.session.get(Battery, device_id)

    def get_generator(self, device_id: "int") -> "Generator | None":
        return self.session.get(Generator, device_id)

    def get_constant_action_device(self, device_id: "int") -> "ConstantActionDevice | None":
        return self.session.get(ConstantActionDevice, device_id)

    def get_variable_action_device(self, device_id: "int") -> "VariableActionDevice | None":
        return self.session.get(VariableActionDevice, device_id)

    def get_all_devices(self) -> "list[Device]":
        return self.session.query(Device).all()

    def get_all_batteries(self) -> "list[Battery]":
        return self.session.query(Battery).all()

    def get_all_generators(self) -> "list[Generator]":
        return self.session.query(Generator).all()

    def get_all_constant_action_devices(self) -> "list[ConstantActionDevice]":
        return self.session.query(ConstantActionDevice).all()

    def get_all_variable_action_devices(self) -> "list[VariableActionDevice]":
        return self.session.query(VariableActionDevice).all()

    def add_device(self, device: "Device") -> "int":
        self.session.add(device)
        self.session.flush()  # Ensure the ID is generated
        return device.id

    def remove_device(self, device_id: "int") -> "None":
        device = self.get_device(device_id)
        if device:
            self.session.delete(device)

    def get_all_device_ids(self) -> "list[int]":
        stmt = select(Device.id)
        return [row[0] for row in self.session.execute(stmt).all()]

    def get_all_battery_ids(self) -> "list[int]":
        stmt = select(Battery.id)
        return [row[0] for row in self.session.execute(stmt).all()]

    def get_all_generator_ids(self) -> "list[int]":
        stmt = select(Generator.id)
        return [row[0] for row in self.session.execute(stmt).all()]

    def get_all_constant_action_device_ids(self) -> "list[int]":
        stmt = select(ConstantActionDevice.id)
        return [row[0] for row in self.session.execute(stmt).all()]

    def get_all_variable_action_device_ids(self) -> "list[int]":
        stmt = select(VariableActionDevice.id)
        return [row[0] for row in self.session.execute(stmt).all()]
