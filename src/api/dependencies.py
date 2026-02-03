from fastapi import Depends
from uow import SqlAlchemyUnitOfWork, IUnitOfWork
from database import SessionLocal
from collections.abc import Generator
from services.controller_service import IControllerService
from services.interactor_service import IInteractorService
from instances import controller_service_instance, interactor_service_instance
from device_manager import IDeviceManager, DeviceManager


def get_uow() -> Generator[IUnitOfWork, None, None]:
    uow = SqlAlchemyUnitOfWork(SessionLocal)
    with uow:
        yield uow


def get_controller_service() -> IControllerService:
    return controller_service_instance


def get_interactor_service() -> IInteractorService:
    return interactor_service_instance


# device manager depends on interactor, controller, and unit of work
def get_device_manager(
    interactor_service: IInteractorService = Depends(get_interactor_service),
    controller_service: IControllerService = Depends(get_controller_service),
    uow: IUnitOfWork = Depends(get_uow),
) -> IDeviceManager:
    return DeviceManager(
        interactor_service=interactor_service,
        controller_service=controller_service,
        uow=uow,
    )
