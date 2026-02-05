from abc import ABC, abstractmethod
from typing import Callable, List, Optional, Type, Any
from sqlalchemy.orm import Session, sessionmaker
from typing import Protocol
from services.controller_service import IControllerService
from services.device_service import IDeviceService, SqlAlchemyDeviceService
from services.interactor_service import IInteractorService


class IUnitOfWork(ABC):
    device_service: IDeviceService
    interactor_service: IInteractorService
    controller_service: IControllerService

    def __enter__(self) -> "IUnitOfWork":
        pass

    def __exit__(self, exc_type, exc_value, traceback) -> None:
        if exc_type is None:
            try:
                self._commit()
            except Exception:
                self._rollback()
                raise
        else:
            self._rollback()

    @abstractmethod
    def _rollback(self) -> None:
        """Rollback all changes made since the last commit."""
        ...

    @abstractmethod
    def _commit(self) -> None:
        """Commit all changes to the database."""
        ...


class SqlAlchemyUnitOfWork(IUnitOfWork):
    def __init__(self, session_factory: sessionmaker, interactor_service: IInteractorService, controller_service: IControllerService):
        self.session_factory = session_factory
        self.session: Optional[Session] = None
        self.device_service: Optional[IDeviceService] = None
        self.interactor_service = interactor_service
        self.controller_service = controller_service

    def __enter__(self) -> "SqlAlchemyUnitOfWork":
        super().__enter__()
        self.session = self.session_factory()
        self.device_service = SqlAlchemyDeviceService(self.session)
        return self

    def _rollback(self) -> None:
        if self.session:
            self.session.rollback()
        self.interactor_service.rollback()
        self.controller_service.rollback()

    def _commit(self) -> None:
        if self.session:
            self.session.commit()
        self.interactor_service.commit()
        self.controller_service.commit()

    def __exit__(self, exc_type: Optional[Type[BaseException]],
                 exc_val: Optional[BaseException],
                 exc_tb: Any) -> None:
        super().__exit__(exc_type, exc_val, exc_tb)
        if self.session:
            self.session.close()
