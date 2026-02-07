"""FastAPI dependency wiring for services and Unit of Work.

Provides:
- get_uow: yields a Unit of Work per-request
- get_controller_service / get_interactor_service: shared singleton instances
- get_device_manager: composes dependencies into a DeviceManager
"""
from fastapi import Depends
from uow import SqlAlchemyUnitOfWork, IUnitOfWork
from database import SessionLocal
from collections.abc import Generator
from services.controller_service import IControllerService
from services.interactor_service import IInteractorService
from services.orchestrator_service import IOrchestratorService
from instances import controller_service_instance, interactor_service_instance, orchestrator_service_instance
from device_manager import IDeviceManager, DeviceManager


def get_uow() -> Generator[IUnitOfWork, None, None]:
    """Yield a Unit of Work (UoW) scoped to the request.

    Behavior:
    - Creates a SqlAlchemyUnitOfWork using the configured SessionLocal factory.
    - Enters the UoW context (opens a SQLAlchemy Session and wires SqlAlchemyDeviceService).
    - Yields the UoW to the dependency consumer (e.g., route handlers).
    - On dependency teardown (after the route returns), exits the UoW:
      - If no exception occurred: commits device changes and commits controller/interactor services.
      - If an exception occurred: rolls back device changes and rolls back controller/interactor services.
      - Always closes the SQLAlchemy Session.

    Notes:
    - The commit/rollback for interactor/controller services delegates to in-memory RollbackMap.
    - Database transaction management is handled by the UoW using the underlying Session.
    """
    uow = SqlAlchemyUnitOfWork(SessionLocal)
    with uow:
        yield uow


def get_controller_service() -> IControllerService:
    """Return the application-scoped controller service singleton."""
    return controller_service_instance


def get_interactor_service() -> IInteractorService:
    """Return the application-scoped interactor service singleton."""
    return interactor_service_instance


def get_orchestrator_service() -> IOrchestratorService:
    """Return the application-scoped orchestrator service singleton."""
    return orchestrator_service_instance

# device manager depends on interactor, controller, and unit of work


def get_device_manager(
    interactor_service: IInteractorService = Depends(get_interactor_service),
    controller_service: IControllerService = Depends(get_controller_service),
    uow: IUnitOfWork = Depends(get_uow),
) -> IDeviceManager:
    """Compose a DeviceManager using DI-provided services and the current UoW."""
    return DeviceManager(
        interactor_service=interactor_service,
        controller_service=controller_service,
        uow=uow,
    )
