from sqlalchemy import create_engine
from sqlalchemy.orm import Session

from database.base import Base

# 1. Create the engine
# 'echo=True' will log all SQL statements to your terminal (great for debugging)
engine = create_engine("sqlite:///database.db", echo=True)


# This looks at all classes inheriting from 'Base' and creates tables in the .db file
def init_db():
    Base.metadata.create_all(engine)
