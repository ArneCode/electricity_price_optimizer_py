from abc import ABC, abstractmethod
from device import Device
from services.controller_service import IControllerService
from services.interactor_service import IInteractorService
from uow import IUnitOfWork


class IDeviceManager(ABC):
    interactor_service: IInteractorService
    controller_service: IControllerService
    uow: IUnitOfWork

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


class DeviceManager(IDeviceManager):
    def __init__(self,
                 interactor_service: IInteractorService,
                 controller_service: IControllerService,
                 uow: IUnitOfWork):
        self.interactor_service = interactor_service
        self.controller_service = controller_service
        self.uow = uow

    def get_device(self, device_id: int) -> Device | None:
        return self.uow.devices.get_device(device_id)

    def add_device(self, device: Device) -> int:
        id = self.uow.devices.add_device(device)
        # TODO: pass actual interactor and controller objects
        self.interactor_service.add_interactor(...)
        self.controller_service.add_controller(...)
        return id

    def remove_device(self, device_id: int) -> None:
        self.uow.devices.remove_device(device_id)
        self.interactor_service.remove_interactor(device_id)
        self.controller_service.remove_controller(device_id)
