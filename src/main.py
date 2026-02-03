from sqlalchemy.orm import Session
from database import engine, init_db
from device import *

init_db()

# create a new session

with Session(engine) as session:
    # create a new battery
    new_battery = Battery(
        name="Home Battery",
        capacity=WattHour(10000),
        current_charge=WattHour(5000),
        max_charge_rate=Watt(2000),
        max_discharge_rate=Watt(2000),
        efficiency=0.9
    )

    # add the battery to the session
    session.add(new_battery)

    # commit the transaction
    session.commit()

    print("Battery added with ID:", new_battery.id)
