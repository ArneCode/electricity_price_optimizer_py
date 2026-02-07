from datetime import datetime, timedelta
from fastapi import APIRouter, Depends, status
from api.dependencies import get_device_manager
from device_manager import IDeviceManager
from device import Battery, VariableActionDevice, VariableAction, ConstantActionDevice, ConstantAction, GeneratorPV
from electricity_price_optimizer_py.units import WattHour, Watt
from services.orchestrator_service import OrchestratorService

router = APIRouter(prefix="/orchestrator", tags=["orchestrator"])


@router.post("/test", status_code=status.HTTP_200_OK)
def test_orchestrator(manager: IDeviceManager = Depends(get_device_manager)) -> dict:
    # Seed sample devices
    # 1) Battery
    battery = Battery(
        name="Test Battery",
        capacity=WattHour(5000.0),
        current_charge=WattHour(2500.0),
        max_charge_rate=Watt(500.0),
        max_discharge_rate=Watt(500.0),
        efficiency=0.95,
        # ...existing code...
    )
    manager.add_battery(battery)

    # 2) Constant action device with one action
    cad = ConstantActionDevice(name="Washer")
    cad.actions.append(
        ConstantAction(
            start_from=datetime.now(),
            end_before=datetime.now() + timedelta(hours=6),
            duration=timedelta(hours=2),
            consumption=Watt(300.0),
            # ...existing code...
        )
    )
    manager.add_constant_action_device(cad)

    # 3) Variable action device with one action
    vad = VariableActionDevice(name="EV Charger")
    vad.actions.append(
        VariableAction(
            start=datetime.now(),
            end=datetime.now() + timedelta(hours=8),
            total_consumption=WattHour(7000.0),
            max_consumption=Watt(2000.0),
            # ...existing code...
        )
    )
    manager.add_variable_action_device(vad)

    # 4) PV generator
    pv = GeneratorPV(name="Roof PV")
    manager.add_generator(pv)

    # Run orchestrator
    orchestrator = OrchestratorService()
    orchestrator.run_optimization(manager)
    schedule = orchestrator.get_schedule()

    # Return a minimal summary
    return {
        "message": "Optimization executed",
        "device_count": len(manager.get_device_service().get_all_devices()),
    }
