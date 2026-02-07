"""Application-scoped singleton service instances.

Provides shared InteractorService and ControllerService objects for dependency injection.
Not multiprocessing-safe; consider per-request construction if needed.
"""
from services.interactor_service import InteractorService
from services.controller_service import ControllerService
interactor_service_instance = InteractorService()
controller_service_instance = ControllerService()
