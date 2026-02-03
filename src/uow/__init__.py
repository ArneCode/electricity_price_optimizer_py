from typing import Callable, List, Optional, Type, Any
from sqlalchemy.orm import Session, sessionmaker
from typing import Protocol
from services.device_service import IDeviceService, SqlAlchemyDeviceService


class IUnitOfWork(Protocol):
    devices: Optional[IDeviceService]

    def __enter__(self) -> "IUnitOfWork":
        ...

    def __exit__(self, exc_type, exc_value, traceback) -> None:
        ...


class SqlAlchemyUnitOfWork:
    def __init__(self, session_factory: sessionmaker):
        self.session_factory = session_factory
        self.session: Optional[Session] = None
        self.devices: Optional[IDeviceService] = None

    def __enter__(self) -> "SqlAlchemyUnitOfWork":
        self.session = self.session_factory()
        self.devices = SqlAlchemyDeviceService(self.session)
        return self

    def __exit__(self, exc_type: Optional[Type[BaseException]],
                 exc_val: Optional[BaseException],
                 exc_tb: Any) -> None:
        if self.session:
            if exc_type is None:
                try:
                    self.session.commit()
                except Exception:
                    self.session.rollback()
                    raise
            else:
                self.session.rollback()
            self.session.close()
