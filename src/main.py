from api.orchestrator import router as orchestrator_router
from fastapi import FastAPI
import uvicorn
from sqlalchemy.orm import Session
from database import engine, init_db
from device import *
from electricity_price_optimizer_py import OptimizerContext, run_simulated_annealing

init_db()

# create a new session

# main.py

app = FastAPI()

# Include the orchestrator router
app.include_router(orchestrator_router)

if __name__ == "__main__":
    uvicorn.run(app, host="127.0.0.1", port=8000)
